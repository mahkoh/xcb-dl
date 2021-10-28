#!/usr/bin/env python3

import sys
import re

# Jump to the bottom of this file for the main routine

# Some hacks to make the API more readable, and to keep backwards compability
_cname_re = re.compile('([A-Z0-9][a-z]+|[A-Z0-9]+(?![a-z])|[a-z]+)')
_cname_special_cases = {'DECnet': 'decnet'}

_to_snake_case_re = re.compile(r'(?<![\^A-Z])(?=[A-Z])')

_extension_special_cases = ['XPrint', 'XCMisc', 'BigRequests']

_c_keywords = {
    'type': 'type_',
    'match': 'match_',
}

_hlines = []
_hlevel = 0
_clines = []
_clevel = 0
_ns = None

# global variable to keep track of serializers and
# switch data types due to weird dependencies
finished_serializers = []
finished_sizeof = []
finished_switch = []

# keeps enum objects so that we can refer to them when generating manpages.
enums = {}

manpaths = False


def to_snake_case(s):
    return _to_snake_case_re.sub('_', s).lower()


def _h(fmt, *args):
    '''
    Writes the given line to the header file.
    '''
    _hlines[_hlevel].append(fmt % args)


def _h_setlevel(idx):
    '''
    Changes the array that header lines are written to.
    Supports writing different sections of the header file.
    '''
    global _hlevel
    while len(_hlines) <= idx:
        _hlines.append([])
    _hlevel = idx


def _n_item(str):
    '''
    Does C-name conversion on a single string fragment.
    Uses a regexp with some hard-coded special cases.
    '''
    if str in _cname_special_cases:
        return _cname_special_cases[str]
    else:
        split = _cname_re.finditer(str)
        name_parts = [match.group(0) for match in split]
        return '_'.join(name_parts)


def _cpp(str):
    '''
    Checks for certain C++ reserved words and fixes them.
    '''
    if str in _c_keywords:
        return _c_keywords[str]
    else:
        return str


def _ext(str):
    '''
    Does C-name conversion on an extension name.
    Has some additional special cases on top of _n_item.
    '''
    if str in _extension_special_cases:
        return _n_item(str).lower()
    else:
        return str.lower()


def _n(list):
    '''
    Does C-name conversion on a tuple of strings.
    Different behavior depending on length of tuple, extension/not extension, etc.
    Basically C-name converts the individual pieces, then joins with underscores.
    '''
    if len(list) == 1:
        parts = list
    elif len(list) == 2:
        parts = [list[0], _n_item(list[1])]
    elif _ns.is_ext:
        parts = [list[0], _ext(list[1])] + [_n_item(i) for i in list[2:]]
    else:
        parts = [list[0]] + [_n_item(i) for i in list[1:]]
    return '_'.join(parts).lower()


c_type_to_rust = {
    'uint8_t': 'u8',
    'uint16_t': 'u16',
    'uint32_t': 'u32',
    'uint64_t': 'u64',
    'int8_t': 'i8',
    'int16_t': 'i16',
    'int32_t': 'i32',
    'int64_t': 'i64',
    'char': 'c_char',
    'float': 'f32',
    'double': 'f64',
    'void': 'c_void',
}


def _t(list):
    '''
    Does C-name conversion on a tuple of strings representing a type.
    Same as _n but adds a "_t" on the end.
    '''
    if len(list) == 1:
        parts = list
        if list[0] in c_type_to_rust:
            return c_type_to_rust[list[0]]
    elif len(list) == 2:
        parts = [list[0], _n_item(list[1]), 't']
    elif _ns.is_ext:
        parts = [list[0], _ext(list[1])] + [_n_item(i) for i in list[2:]] + ['t']
    else:
        parts = [list[0]] + [_n_item(i) for i in list[1:]] + ['t']
    return '_'.join(parts).lower()


def add_sym(name, params, rv='()', fn=True):
    params_str = ''
    args_str = ''
    for pname, ty in params:
        params_str += f'{pname}: {ty}, '
        args_str += f'{pname}, '

    invocation = ''
    unsafe = ''
    if fn:
        invocation = f'({args_str})'
        unsafe = 'unsafe '
    body = f'sym!(self, {name}){invocation}'
    if not fn:
        body = 'unsafe { ' + body + ' }'

    if rv == '()':
        fnrv = ''
    else:
        fnrv = ' -> ' + rv

    _h_setlevel(1)
    if fn:
        _h(f'    {name}: LazySymbol<unsafe fn({params_str}){fnrv}>,')
    else:
        _h(f'    {name}: LazySymbol<{rv}>,')

    _h_setlevel(2)
    _h('    #[inline]')
    _h(f'    pub {unsafe}fn {name}(&self, {params_str}){fnrv} {{ {body} }}')
    _h('')
    _h(f'    /// Returns `true` iff the symbol `{name}` could be loaded.')
    _h(f'    #[cfg(feature = "has_symbol")]')
    _h(f'    pub fn has_{name}(&self) -> bool {{ has_sym!(self, {name}) }}')

    _h_setlevel(3)
    _h(f'        assert!(lib.has_{name}());')


def c_open(self):
    '''
    Exported function that handles module open.
    Opens the files and writes out the auto-generated comment, header file includes, etc.
    '''
    global _ns
    _ns = self.namespace
    _ns.c_ext_global_name = _n(_ns.prefix + ('id',))

    obj_names = {
        'bigreq': 'Xcb',
        'xc_misc': 'Xcb',
        'xproto': 'Xcb',
        'xf86dri': 'XcbXf86dri',
    }

    if _ns.header in obj_names:
        _ns.rust_obj_name = obj_names[_ns.header]
    else:
        _ns.rust_obj_name = 'Xcb' + _ns.header.title()

    no_feature = {'bigreq', 'xc_misc', 'xproto'}
    if _ns.header in no_feature:
        feature = None
    else:
        feature = f'#[cfg(feature = "xcb_{_ns.header.lower()}")]'

    local_obj_name = f'{_ns.rust_obj_name}{_ns.header.title()}'

    # Build the type-name collision avoidance table used by c_enum
    build_collision_table()

    _h_setlevel(0)
    _h('// This file was generated using generate.py. Do not edit.')
    _h('#![allow(unused_macros)]')
    _h('')
    _h('use crate::ffi::*;')
    _h('use crate::*;')
    _h('use crate::lazy::*;')
    _h('use std::os::raw::*;')
    _h('')

    _h_setlevel(1)
    _h('')
    if feature:
        _h(feature)
    _h(f'pub(crate) struct {local_obj_name} {{')

    _h_setlevel(2)
    _h('')
    _h('macro_rules! sym {')
    _h('    ($self:expr, $f:ident) => {')
    _h(f'        $self.{_ns.header}.$f.get(&$self.lib, concat!(stringify!($f), "\\0"))')
    _h('    };')
    _h('}')
    _h('')
    _h('macro_rules! has_sym {')
    _h('    ($self:expr, $f:ident) => {')
    _h('        unsafe {')
    _h(f'            $self.{_ns.header}.$f.exists(&$self.lib, concat!(stringify!($f), "\\0"))')
    _h('        }')
    _h('    };')
    _h('}')
    _h('')
    if feature:
        _h(feature)
    _h(f'impl {_ns.rust_obj_name} {{')

    _h_setlevel(3)
    _h('')
    if feature:
        _h(feature)
    _h('#[cfg(all(test, feature = "has_symbol"))]')
    _h('mod test {')
    _h('    #[test]')
    _h('    fn has_all() {')
    _h(f'        let lib = unsafe {{ crate::{_ns.rust_obj_name}::load().unwrap() }};')

    if _ns.is_ext:
        _h_setlevel(0)
        _h(f'/// The name of the `{_ns.ext_name}` extension.')
        _h(f'pub const {_n(_ns.prefix + ("name",)).upper()}: &[u8] = b"{_ns.ext_xname}";')
        _h(f'')
        _h(f'/// The name of the `{_ns.ext_name}` extension.')
        _h(f'pub const {_n(_ns.prefix + ("name", "str")).upper()}: &str = "{_ns.ext_xname}";')
        _h(f'')
        _h_setlevel(2)
        _h(f'/// The libxcb identifier of the `{_ns.ext_name}` extension.')
        add_sym(_ns.c_ext_global_name, [], rv='*mut xcb_extension_t', fn=False)


def c_close(self):
    '''
    Exported function that handles module close.
    Writes out all the stored content lines, then closes the files.
    '''
    _h_setlevel(1)
    _h(f'}}')

    _h_setlevel(2)
    _h(f'}}')

    _h_setlevel(3)
    _h(f'    }}')
    _h(f'}}')

    # Write header file
    hfile = open('src/headers/%s.rs' % _ns.header, 'w')
    for list in _hlines:
        for line in list:
            hfile.write(line)
            hfile.write('\n')
    hfile.close()


def build_collision_table():
    global namecount
    namecount = {}

    for v in module.types.values():
        name = _t(v[0])
        namecount[name] = (namecount.get(name) or 0) + 1


def c_enum(self, name):
    '''
    Exported function that handles enum declarations.
    '''

    enums[name] = self

    tname = _t(name)
    if namecount[tname] > 1:
        tname = _t(name + ('enum',))

    _h_setlevel(0)
    _h('')
    _h(f'/// The `{_opcode_name(name)}` enum.')
    _h('///')
    _h('/// This enum has the following variants:')
    _h('///')
    for (enam, eval) in self.values:
        _h(f'/// - [`{_opcode_name(name)}::{enam}`]({_n(name + (enam,)).upper()})')
    _h(f'pub type {tname} = u32;')

    last_given = None
    num_since_last_given = 0

    for (enam, eval) in self.values:
        if eval == '':
            if last_given:
                val = f'{last_given} + {num_since_last_given}'
            else:
                val = f'{num_since_last_given}'
            num_since_last_given += 1
        else:
            val = eval
            num_since_last_given = 1
        _h(f'/// The `{_opcode_name(name)}::{enam}` enum variant.')
        _h(f'///')
        _h(f'/// This is a variant of [`{tname}`].')
        _h(f'pub const {_n(name + (enam,)).upper()}: {tname} = {val};')


def _c_type_setup(self, name, postfix):
    '''
    Sets up all the C-related state by adding additional data fields to
    all Field and Type objects.  Here is where we figure out most of our
    variable and function names.

    Recurses into child fields and list member types.
    '''
    # Do all the various names in advance
    self.c_type = _t(name + postfix)

    self.c_iterator_type = _t(name + ('iterator',))
    self.c_next_name = _n(name + ('next',))
    self.c_end_name = _n(name + ('end',))

    self.c_request_name = _n(name)
    self.c_checked_name = _n(name + ('checked',))
    self.c_unchecked_name = _n(name + ('unchecked',))
    self.c_reply_name = _n(name + ('reply',))
    self.c_reply_type = _t(name + ('reply',))
    self.c_cookie_type = _t(name + ('cookie',))
    self.c_reply_fds_name = _n(name + ('reply_fds',))

    self.c_need_aux = False
    self.c_need_serialize = False
    self.c_need_sizeof = False

    self.c_aux_name = _n(name + ('aux',))
    self.c_aux_checked_name = _n(name + ('aux', 'checked'))
    self.c_aux_unchecked_name = _n(name + ('aux', 'unchecked'))
    self.c_serialize_name = _n(name + ('serialize',))
    self.c_unserialize_name = _n(name + ('unserialize',))
    self.c_unpack_name = _n(name + ('unpack',))
    self.c_sizeof_name = _n(name + ('sizeof',))

    # special case: structs where variable size fields are followed by fixed size fields
    self.c_var_followed_by_fixed_fields = False

    if self.is_switch:
        self.c_need_serialize = True
        self.c_container = 'struct'
        for bitcase in self.bitcases:
            bitcase.c_field_name = _cpp(bitcase.field_name)
            bitcase_name = bitcase.field_type if bitcase.type.has_name else name
            _c_type_setup(bitcase.type, bitcase_name, ())

    elif self.is_container:

        self.c_container = 'union' if self.is_union else 'struct'
        prev_varsized_field = None
        prev_varsized_offset = 0
        first_field_after_varsized = None

        for field in self.fields:
            if field.type.is_event:
                field.c_field_type = _t(field.field_type + ('event',))
            else:
                field.c_field_type = _t(field.field_type)
            if field.type.nmemb and field.type.nmemb > 1:
                field.c_field_type = f'[{field.c_field_type}; {field.type.nmemb}]'

            field.has_c_field_const_type = False if field.type.nmemb == 1 else True
            field.c_field_name = _cpp(field.field_name)
            field.has_c_pointer = False if field.type.nmemb == 1 else True

            # correct the c_pointer field for variable size non-list types
            if not field.type.fixed_size() and not field.has_c_pointer:
                field.has_c_pointer = True
            if field.type.is_list and not field.type.member.fixed_size():
                field.has_c_pointer = True

            if field.type.is_switch:
                field.has_c_pointer = True
                field.has_c_field_const_type = True
                self.c_need_aux = True

            if not field.type.fixed_size() and not field.type.is_case_or_bitcase and field.wire:
                self.c_need_sizeof = True

            field.c_iterator_type = _t(field.field_type + ('iterator',))  # xcb_fieldtype_iterator_t
            field.c_iterator_name = _n(name + (field.field_name, 'iterator'))  # xcb_container_field_iterator
            field.c_accessor_name = _n(name + (field.field_name,))  # xcb_container_field
            field.c_length_name = _n(name + (field.field_name, 'length'))  # xcb_container_field_length
            field.c_end_name = _n(name + (field.field_name, 'end'))  # xcb_container_field_end

            field.prev_varsized_field = prev_varsized_field
            field.prev_varsized_offset = prev_varsized_offset

            if prev_varsized_offset == 0:
                first_field_after_varsized = field
            field.first_field_after_varsized = first_field_after_varsized

            if field.type.fixed_size():
                if field.wire:
                    prev_varsized_offset += field.type.size
                # special case: intermixed fixed and variable size fields
                if prev_varsized_field is not None and not field.type.is_pad and field.wire:
                    if not self.is_union:
                        self.c_need_serialize = True
                        self.c_var_followed_by_fixed_fields = True
            else:
                self.last_varsized_field = field
                prev_varsized_field = field
                prev_varsized_offset = 0

            if self.c_var_followed_by_fixed_fields:
                if field.type.fixed_size():
                    field.prev_varsized_field = None

            # recurse into this field this has to be done here, i.e.,
            # after the field has been set up. Otherwise the function
            # _c_helper_fieldaccess_expr will produce garbage or crash
            _c_type_setup(field.type, field.field_type, ())
            if field.type.is_list:
                _c_type_setup(field.type.member, field.field_type, ())
                if (field.type.nmemb is None and field.wire):
                    self.c_need_sizeof = True

    if self.c_need_serialize:
        # when _unserialize() is wanted, create _sizeof() as well for consistency reasons
        self.c_need_sizeof = True

    # as switch does never appear at toplevel,
    # continue here with type construction
    if self.is_switch:
        if self.c_type not in finished_switch:
            finished_switch.append(self.c_type)
            # special: switch C structs get pointer fields for variable-sized members
            _c_complex(self, _opcode_name(name), 'switch')
            for bitcase in self.bitcases:
                bitcase_name = bitcase.type.name if bitcase.type.has_name else name
                _c_accessors(bitcase.type, bitcase_name, bitcase_name)
                # no list with switch as element, so no call to
                # _c_iterator(field.type, field_name) necessary

    if not self.is_case_or_bitcase:
        if self.c_need_serialize:
            if self.c_serialize_name not in finished_serializers:
                finished_serializers.append(self.c_serialize_name)
                _c_serialize('serialize', self)

                # _unpack() and _unserialize() are only needed for special cases:
                #   switch -> unpack
                #   special cases -> unserialize
                if self.is_switch or self.c_var_followed_by_fixed_fields:
                    _c_serialize('unserialize', self)

        if self.c_need_sizeof:
            if self.c_sizeof_name not in finished_sizeof:
                if not module.namespace.is_ext or self.name[:2] == module.namespace.prefix:
                    finished_sizeof.append(self.c_sizeof_name)
                    _c_serialize('sizeof', self)


# Functions for querying field properties
def _c_field_needs_list_accessor(field):
    return field.type.is_list and not field.type.fixed_size()


def _c_field_needs_field_accessor(field):
    if field.type.is_list:
        return False
    else:
        return (field.prev_varsized_field is not None or
                not field.type.fixed_size())


def _c_field_needs_accessor(field):
    return (_c_field_needs_list_accessor(field) or
            _c_field_needs_field_accessor(field))


def _c_field_is_member_of_case_or_bitcase(field):
    return field.parent and field.parent.is_case_or_bitcase


def _c_helper_fieldaccess_expr(prefix, field=None):
    """
    turn prefix, which is a list of tuples (name, separator, Type obj) into a string
    representing a valid field-access-expression in C (based on the context)
    if field is not None, append access to the field as well.

    "separator" is one of the C-operators "." or "->".

    A field access expression can consist of the following components:
    * struct/union member access from a value with the "."-operator
    * struct/union member access from a pointer with "->"-operator
    * function-call of an accessor function:
      This is used when a xcb-field is not contained in a struct.
      This can, e.g., happen for fields after var-sized fields, etc.
    """
    prefix_str = ''
    last_sep = ''
    for name, sep, obj in prefix:
        prefix_str += last_sep + name
        last_sep = sep

    if field is None:
        # add separator for access to a yet unknown field
        prefix_str += last_sep
    else:
        if _c_field_needs_accessor(field):
            if _c_field_is_member_of_case_or_bitcase(field):
                # case members are available in the deserialized struct,
                # so there is no need to use the accessor function
                # (also, their accessor function needs a different arglist
                # so this would require special treatment here)
                # Therefore: Access as struct member
                prefix_str += last_sep + _cpp(field.field_name)
            else:
                # Access with the accessor function
                prefix_str = field.c_accessor_name + "(" + prefix_str + ")"
        else:
            # Access as struct member
            prefix_str += last_sep + _cpp(field.field_name)

    return prefix_str


def _c_helper_field_mapping(complex_type, prefix, flat=False):
    """
    generate absolute names, based on prefix, for all fields starting from complex_type
    if flat == True, nested complex types are not taken into account
    """
    all_fields = {}
    if complex_type.is_switch:
        for b in complex_type.bitcases:
            if b.type.has_name:
                switch_name, switch_sep, switch_type = prefix[-1]
                bitcase_prefix = prefix + [(b.type.name[-1], '.', b.type)]
            else:
                bitcase_prefix = prefix

            if (flat and not b.type.has_name) or not flat:
                all_fields.update(_c_helper_field_mapping(b.type, bitcase_prefix, flat))
    else:
        for f in complex_type.fields:
            fname = _c_helper_fieldaccess_expr(prefix, f)
            if f.field_name in all_fields:
                raise Exception("field name %s has been registered before" % f.field_name)

            all_fields[f.field_name] = (fname, f)
            if f.type.is_container and not flat:
                if f.type.is_case_or_bitcase and not f.type.has_name:
                    new_prefix = prefix
                elif f.type.is_switch and len(f.type.parents) > 1:
                    # nested switch gets another separator
                    new_prefix = prefix + [(f.c_field_name, '.', f.type)]
                else:
                    new_prefix = prefix + [(f.c_field_name, '->', f.type)]
                all_fields.update(_c_helper_field_mapping(f.type, new_prefix, flat))

    return all_fields


def _c_helper_resolve_field_names(prefix):
    """
    get field names for all objects in the prefix array
    """
    all_fields = {}
    tmp_prefix = []
    # look for fields in the remaining containers
    for idx, (name, sep, obj) in enumerate(prefix):
        if '' == sep:
            # sep can be preset in prefix, if not, make a sensible guess
            sep = '.' if (obj.is_switch or obj.is_case_or_bitcase) else '->'
            # exception: 'toplevel' object (switch as well!) always have sep '->'
            sep = '->' if idx < 1 else sep
        if not obj.is_case_or_bitcase or (obj.is_case_or_bitcase and obj.has_name):
            tmp_prefix.append((name, sep, obj))
        all_fields.update(_c_helper_field_mapping(obj, tmp_prefix, flat=True))

    return all_fields


def get_expr_fields(self):
    """
    get the Fields referenced by switch or list expression
    """

    def get_expr_field_names(expr):
        if expr.op is None or expr.op == 'calculate_len':
            if expr.lenfield_name is not None:
                return [expr.lenfield_name]
            else:
                # constant value expr
                return []
        else:
            if expr.op == '~':
                return get_expr_field_names(expr.rhs)
            elif expr.op == 'popcount':
                return get_expr_field_names(expr.rhs)
            elif expr.op == 'sumof':
                # sumof expr references another list,
                # we need that list's length field here
                field = None
                for f in expr.lenfield_parent.fields:
                    if f.field_name == expr.lenfield_name:
                        field = f
                        break
                if field is None:
                    raise Exception("list field '%s' referenced by sumof not found" % expr.lenfield_name)
                # referenced list + its length field
                return [expr.lenfield_name] + get_expr_field_names(field.type.expr)
            elif expr.op == 'enumref':
                return []
            else:
                return get_expr_field_names(expr.lhs) + get_expr_field_names(expr.rhs)

    # get_expr_field_names()

    # resolve the field names with the parent structure(s)
    unresolved_fields_names = get_expr_field_names(self.expr)

    # construct prefix from self
    prefix = [('', '', p) for p in self.parents]
    if self.is_container:
        prefix.append(('', '', self))

    all_fields = _c_helper_resolve_field_names(prefix)
    resolved_fields_names = [x for x in unresolved_fields_names if x in all_fields]
    if len(unresolved_fields_names) != len(resolved_fields_names):
        raise Exception("could not resolve all fields for %s" % self.name)

    resolved_fields = [all_fields[n][1] for n in resolved_fields_names]
    return resolved_fields


def resolve_expr_fields(complex_obj):
    """
    find expr fields appearing in complex_obj and descendents that cannot be resolved within complex_obj
    these are normally fields that need to be given as function parameters
    """
    all_fields = []
    expr_fields = []
    unresolved = []

    for field in complex_obj.fields:
        all_fields.append(field)
        if field.type.is_switch or field.type.is_list:
            expr_fields += get_expr_fields(field.type)
        if field.type.is_container:
            expr_fields += resolve_expr_fields(field.type)

    # try to resolve expr fields
    for e in expr_fields:
        if e not in all_fields and e not in unresolved:
            unresolved.append(e)
    return unresolved


def resolve_expr_fields_list(self, parents):
    """
    Find expr fields appearing in a list and descendents
    that cannot be resolved within the parents of the list.
    These are normally fields that need to be given as function parameters
    for length and iterator functions.
    """
    all_fields = []
    expr_fields = get_expr_fields(self)
    unresolved = []
    dont_resolve_this = ''
    for complex_obj in parents:
        for field in complex_obj.fields:
            if field.type.is_list and field.type.expr.op == 'calculate_len':
                dont_resolve_this = field.type.expr.lenfield_name
            if field.wire:
                all_fields.append(field)

    # try to resolve expr fields
    for e in expr_fields:
        if e not in all_fields and e not in unresolved and e.field_name != dont_resolve_this:
            unresolved.append(e)

    return unresolved


def get_serialize_params(context, self, buffer_var='_buffer', aux_var='_aux'):
    """
    functions like _serialize(), _unserialize(), and _unpack() sometimes need additional parameters:
    E.g. in order to unpack switch, extra parameters might be needed to evaluate the switch
    expression. This function tries to resolve all fields within a structure, and returns the
    unresolved fields as the list of external parameters.
    """

    def add_param(params, param):
        if param not in params:
            params.append(param)
        else:
            raise 'Should not happen'

    # collect all fields into param_fields
    param_fields = []

    for field in self.fields:
        if field.visible:
            # the field should appear as a parameter in the function call
            param_fields.append(field)

    # in case of switch, parameters always contain any fields referenced in the switch expr
    # we do not need any variable size fields here, as the switch data type contains both
    # fixed and variable size fields
    if self.is_switch:
        param_fields = get_expr_fields(self)

    # _serialize()/_unserialize()/_unpack() function parameters
    # note: don't use set() for params, it is unsorted
    params = []
    parameter = ''
    if self.is_list:
        parameter = self.type.expr.lenfield_name
    # 1. the parameter for the void * buffer
    if 'serialize' == context:
        params.append((buffer_var, '*mut *mut c_void'))
    elif context in ('unserialize', 'unpack', 'sizeof'):
        params.append((buffer_var, '*const c_void'))

    # 2. any expr fields that cannot be resolved within self and descendants
    unresolved_fields = resolve_expr_fields(self)
    for f in unresolved_fields:
        add_param(params, (f.c_field_name, f.c_field_type))

    # 3. param_fields contain the fields necessary to evaluate the switch expr or any other fields
    #    that do not appear in the data type struct
    for p in param_fields:
        if self.is_switch:
            if p.has_c_pointer:
                if p.has_c_field_const_type:
                    ty = f'*const {p.c_field_type}'
                else:
                    ty = f'*mut {p.c_field_type}'
            else:
                ty = p.c_field_type
            add_param(params, (p.c_field_name, ty))
        else:
            if p.visible and not p.wire and not p.auto and p.field_name != parameter:
                add_param(params, (p.c_field_name, p.c_field_type))

    # 4. aux argument
    if 'serialize' == context:
        add_param(params, (aux_var, f'*const {self.c_type}'))
    elif 'unserialize' == context:
        add_param(params, (aux_var, f'*mut *mut {self.c_type}'))
    elif 'unpack' == context:
        add_param(params, (aux_var, f'*mut {self.c_type}'))

    # 5. switch contains all variable size fields as struct members
    #    for other data types though, these have to be supplied separately
    #    this is important for the special case of intermixed fixed and
    #    variable size fields
    if not self.is_switch and 'serialize' == context:
        for p in param_fields:
            if not p.type.fixed_size():
                if p.has_c_field_const_type:
                    ty = f'*const {p.c_field_type}'
                else:
                    ty = f'*mut {p.c_field_type}'
                add_param(params, (p.c_field_name, ty))

    return params


def _c_get_additional_type_params(type):
    """
    compute list of additional params for functions created for the given type
    """
    if type.is_simple:
        return []
    else:
        params = get_serialize_params('sizeof', type)
        return params[1:]


def _c_serialize(context, self):
    """
    depending on the context variable, generate _serialize(), _unserialize(), _unpack(), or _sizeof()
    for the ComplexType variable self
    """
    if self.is_switch and context == 'unserialize':
        context = 'unpack'

    cases = {'serialize': self.c_serialize_name,
             'unserialize': self.c_unserialize_name,
             'unpack': self.c_unpack_name,
             'sizeof': self.c_sizeof_name}
    func_name = cases[context]

    params = get_serialize_params(context, self)

    _h_setlevel(2)
    _h('')
    if context == 'serialize':
        _h(f'/// Serializes a `{self.c_type}` object.')
    elif context == 'unserialize':
        _h(f'/// Deserializes a `{self.c_type}` object.')
        _h(f'///')
        _h(f'/// The object returned in `_aux` should be freed with `libc::free`.')
    elif context == 'unpack':
        _h(f'/// Unpacks a `{self.c_type}` object.')
    elif context == 'sizeof':
        _h(f'/// Computes the size of a `{self.c_type}` object.')
    else:
        raise
    add_sym(func_name, params, rv='c_int')


def impl_default(name):
    _h('')
    _h(f'impl Default for {name} {{')
    _h('    fn default() -> Self {')
    _h('        unsafe { std::mem::MaybeUninit::zeroed().assume_init() }')
    _h('    }')
    _h('}')


def _c_iterator(self, name):
    '''
    Declares the iterator structure and next/end functions for a given type.
    '''
    _h_setlevel(0)
    _h('')
    _h(f'/// An iterator over `{_opcode_name(name)}` objects.')
    _h('#[derive(Copy, Clone, Debug)]')
    _h('#[repr(C)]')
    _h('pub struct %s {', self.c_iterator_type)
    _h('    /// The value of the current iteration.')
    _h(f'    pub data: *mut {self.c_type},')
    _h('    /// The number of elements remaining including this one.')
    _h('    pub rem: c_int,')
    _h('    /// The offset of `data`, in bytes, from the start of the containing object.')
    _h('    pub index: c_int,')
    # add additional params of the type "self" as fields to the iterator struct
    # so that they can be passed to the sizeof-function by the iterator's next-function
    params = _c_get_additional_type_params(self)
    for name, ty in params:
        _h(f'    pub {to_snake_case(name)}: {ty},')
    _h('}')
    impl_default(self.c_iterator_type)

    _h_setlevel(2)
    _h('')
    _h(f'/// Advances a `{self.c_iterator_type}` iterator by 1 element.')
    add_sym(self.c_next_name, [('i', f'*mut {self.c_iterator_type}')])
    _h_setlevel(2)
    _h('')
    _h(f'/// Returns a `xcb_generic_iterator_t` pointing just past the end of a `{self.c_iterator_type}`.')
    add_sym(self.c_end_name, [('i', f'{self.c_iterator_type}')], rv='xcb_generic_iterator_t')


def _c_accessors_field(self, field):
    '''
    Declares the accessor functions for a non-list field that follows a variable-length field.
    '''
    # special case: switch
    switch_obj = self if self.is_switch else None
    if self.is_case_or_bitcase:
        switch_obj = self.parents[-1]

    if switch_obj is None:
        c_type = self.c_type
    else:
        c_type = switch_obj.c_type

    pointer = ''

    if field.type.is_simple:
        return_type = field.c_field_type
    else:
        pointer = ' a pointer to'
        if field.type.is_switch and switch_obj is None:
            return_type = '*mut c_void'
        else:
            return_type = '*mut ' + field.c_field_type

    _h_setlevel(2)
    _h('')
    _h(f'/// Returns{pointer} the `{field.c_field_name}` field of a `{c_type}` struct.')
    add_sym(field.c_accessor_name, [('r', f'*const {c_type}')], rv=return_type)


def _c_accessors_list(self, field):
    '''
    Declares the accessor functions for a list field.
    Declares a direct-accessor function only if the list members are fixed size.
    Declares length and get-iterator functions always.
    '''
    list = field.type

    # special case: switch
    # in case of switch, 2 params have to be supplied to certain accessor functions:
    #   1. the anchestor object (request or reply)
    #   2. the (anchestor) switch object
    # the reason is that switch is either a child of a request/reply or nested in another switch,
    # so whenever we need to access a length field, we might need to refer to some anchestor type
    switch_obj = self if self.is_switch else None
    if self.is_case_or_bitcase:
        switch_obj = self.parents[-1]

    params = []
    fields = {}
    parents = self.parents if hasattr(self, 'parents') else [self]
    # 'R': parents[0] is always the 'toplevel' container type
    params.append(('r', f'*const {parents[0].c_type}'))
    fields.update(_c_helper_field_mapping(parents[0], [('r', '->', parents[0])], flat=True))
    # auxiliary object for 'R' parameters
    R_obj = parents[0]

    if switch_obj is not None:
        # now look where the fields are defined that are needed to evaluate
        # the switch expr, and store the parent objects in accessor_params and
        # the fields in switch_fields

        # 'S': name for the 'toplevel' switch
        toplevel_switch = parents[1]
        params.append(('s', f'*const {toplevel_switch.c_type}'))
        fields.update(_c_helper_field_mapping(toplevel_switch, [('s', '->', toplevel_switch)], flat=True))

        # initialize prefix for everything "below" S
        prefix = [('s', '->', toplevel_switch)]

        # look for fields in the remaining containers
        for p in parents[2:] + [self]:
            # the separator between parent and child is always '.' here,
            # because of nested switch statements
            if not p.is_case_or_bitcase or (p.is_case_or_bitcase and p.has_name):
                prefix.append((p.name[-1], '.', p))
            fields.update(_c_helper_field_mapping(p, prefix, flat=True))

        # auxiliary object for 'S' parameter
        S_obj = parents[1]

    # for functions generated below:
    # * compute list of additional parameters which contains as parameter
    #   any expr fields that cannot be resolved within self and descendants.
    # * and make sure that they are accessed without prefix within the function.
    unresolved_fields = resolve_expr_fields_list(list, parents)
    additional_params = []
    additional_param_names = set();
    for f in unresolved_fields:
        if f.c_field_name not in additional_param_names:
            # add to the list of additional params
            additional_params.append((to_snake_case(f.c_field_name), f.c_field_type))
            # make sure that the param is accessed without prefix within the function
            fields[f.c_field_name] = (f.c_field_name, f)

    _h_setlevel(1)
    if list.member.fixed_size():
        idx = 1 if switch_obj is not None else 0
        _h_setlevel(2)
        _h('')
        _h(f'/// Returns a pointer to the `{field.c_field_name}` field of a `{self.c_type}` struct.')
        add_sym(field.c_accessor_name, [params[idx]], rv=f'*mut {field.c_field_type}')

    if switch_obj is not None:
        params = [
                    ('r', f'*const {R_obj.c_type}'),
                    ('s', f'*const {S_obj.c_type}'),
                ] + additional_params
    else:
        params = [
                     ('r', f'*const {R_obj.c_type}'),
                 ] + additional_params
    _h_setlevel(2)
    _h('')
    _h(f'/// Returns the number of elements of the `{field.c_field_name}` field of a `{self.c_type}` struct.')
    add_sym(field.c_length_name, params, rv='c_int')

    _h_setlevel(2)
    _h('')
    if field.type.member.is_simple:
        _h(f'/// Returns a `xcb_generic_iterator_t` pointing just past the end of the')
        _h(f'/// `{field.c_field_name}` field of a `{self.c_type}` struct.')
        add_sym(field.c_end_name, params, rv='xcb_generic_iterator_t')
    else:
        _h(f'/// Returns an iterator over the elements of the')
        _h(f'/// `{field.c_field_name}` field of a `{self.c_type}` struct.')
        add_sym(field.c_iterator_name, params, rv=field.c_iterator_type)


def _c_accessors(self, name, base):
    '''
    Declares the accessor functions for the fields of a structure.
    '''
    # no accessors for switch itself -
    # switch always needs to be unpacked explicitly
    #    if self.is_switch:
    #        pass
    #    else:
    if True:
        for field in self.fields:
            if not field.type.is_pad:
                if _c_field_needs_list_accessor(field):
                    _c_accessors_list(self, field)
                elif _c_field_needs_field_accessor(field):
                    _c_accessors_field(self, field)


def c_simple(self, name):
    '''
    Exported function that handles cardinal type declarations.
    These are types which are typedef'd to one of the CARDx's, char, float, etc.
    '''
    _c_type_setup(self, name, ())

    if (self.name != name):
        # Typedef
        _h_setlevel(0)
        my_name = _t(name)
        _h('')
        _h(f'/// The `{_opcode_name(name)}` type.')
        _h(f'pub type {my_name} = {_t(self.name)};')

        # Iterator
        _c_iterator(self, name)


def collect_accessors(ty):
    ret = []
    for field in ty.fields:
        if not field.type.is_pad:
            if _c_field_needs_list_accessor(field) or _c_field_needs_field_accessor(field):
                ret.append(to_snake_case(field.c_field_name))
    return ret


def list_accessors(ty):
    accessors = collect_accessors(ty)
    if len(accessors) > 0:
        _h('///')
        _h('/// The following fields can be accessed via accessor functions:')
        _h('///')
        for accessor in accessors:
            _h(f'/// - `{accessor}`')


def _c_complex(self, opcode_name, kind, force_packed=False, has_fds=False):
    '''
    Helper function for handling all structure types.
    Called for all structs, requests, replies, events, errors.
    '''
    _h_setlevel(0)

    def _c_complex_field(self, field):
        name = to_snake_case(field.c_field_name)
        if (field.type.fixed_size() or self.is_union or
                # in case of switch with switch children, don't make the field a pointer
                # necessary for unserialize to work
                (self.is_switch and field.type.is_switch)):
            _h(f'    pub {name}: {field.c_field_type},')
        elif (not field.type.is_pad) or field.type.serialize:
            # serialize everything except pads (unless serialization of pads is enforced by serialize=true)
            _h(f'    pub {name}: *mut {field.c_field_type},')

    if self.is_switch:
        for b in self.bitcases:
            if b.type.has_name:
                name = f'{self.c_type}__{b.c_field_name}'
                _h('')
                _h(f'/// The type of [`{self.c_type}::{b.c_field_name}`].')
                _h(f'///')
                _h(f'/// In libxcb, this type is an anonymous struct.')
                list_accessors(b.type)
                _h('#[derive(Copy, Clone, Debug)]')
                _h('#[repr(C)]')
                _h(f'pub struct {name} {{')
                for field in b.type.fields:
                    _c_complex_field(self, field)
                _h(f'}}')
                impl_default(name)

    packed = ', packed' if force_packed else ''

    debug = ', Debug' if self.c_container == 'struct' else ''

    _h('')
    _h(f'/// The `{opcode_name}` {kind}.')
    if has_fds:
        _h('///')
        _h(f'/// This reply contains file descriptors that can be accessed with [`{self.c_reply_fds_name}`].')
        _h('///')
        _h(f'/// [`{self.c_reply_fds_name}`]: {_ns.rust_obj_name}::{self.c_reply_fds_name}')
    if not self.is_switch:
        list_accessors(self)
    _h(f'#[derive(Copy, Clone{debug})]')
    _h(f'#[repr(C{packed})]')
    _h('pub %s %s {', self.c_container, self.c_type)

    if self.is_switch:
        for b in self.bitcases:
            if b.type.has_name:
                _h(f'    pub {b.c_field_name}: {self.c_type}__{b.c_field_name},')
            else:
                for field in b.type.fields:
                    _c_complex_field(self, field)
    else:
        for field in self.fields:
            if field.wire and (field.type.fixed_size() or self.is_union):
                _c_complex_field(self, field)

    _h('}')
    impl_default(self.c_type)


def c_struct(self, name):
    '''
    Exported function that handles structure declarations.
    '''
    _c_type_setup(self, name, ())
    _c_complex(self, _opcode_name(name), 'struct')
    _c_accessors(self, name, name)
    _c_iterator(self, name)


def c_union(self, name):
    '''
    Exported function that handles union declarations.
    '''
    _c_type_setup(self, name, ())
    _c_complex(self, _opcode_name(name), 'union')
    _c_iterator(self, name)


def _c_request_helper(self, name, void, regular, aux=False):
    '''
    Declares a request function.
    '''

    # Four stunningly confusing possibilities here:
    #
    #   Void            Non-void
    # ------------------------------
    # "req"            "req"
    # 0 flag           CHECKED flag   Normal Mode
    # void_cookie      req_cookie
    # ------------------------------
    # "req_checked"    "req_unchecked"
    # CHECKED flag     0 flag         Abnormal Mode
    # void_cookie      req_cookie
    # ------------------------------

    # Whether we are _checked or _unchecked
    checked = void and not regular
    unchecked = not void and not regular

    # What kind of cookie we return
    func_cookie = 'xcb_void_cookie_t' if void else self.c_cookie_type

    # What our function name is
    func_name = self.c_request_name if not aux else self.c_aux_name
    aux_name = self.c_aux_name
    if checked:
        func_name = self.c_checked_name if not aux else self.c_aux_checked_name
        aux_name = self.c_aux_checked_name
    if unchecked:
        func_name = self.c_unchecked_name if not aux else self.c_aux_unchecked_name
        aux_name = self.c_aux_unchecked_name

    param_fields = []
    wire_fields = []
    serial_fields = []

    for field in self.fields:
        if field.visible:
            # The field should appear as a call parameter
            param_fields.append(field)
        if field.wire and not field.auto:
            # We need to set the field up in the structure
            wire_fields.append(field)
        if field.type.c_need_serialize or field.type.c_need_sizeof:
            serial_fields.append(field)

    aux_h = ' (aux)' if aux else ''
    checked_h = 'unchecked' if void == regular else 'checked'

    _h_setlevel(2)
    _h('')
    _h(f'/// Sends a `{_opcode_name(name)}` request ({checked_h}){aux_h}.')
    if not void:
        _h('///')
        _h('/// This request generates a reply. You must either discard it with')
        _h(f'/// [`discard_reply`] or retrieve it with [`{self.c_reply_name}`].')
        _h('///')
        _h('/// [`discard_reply`]: crate::Xcb::xcb_discard_reply')
        _h(f'/// [`{self.c_reply_name}`]: Self::{self.c_reply_name}')
    if checked:
        _h('///')
        _h('/// This request generates a reply. You must either discard it with')
        _h('/// [`discard_reply`] or retrieve it with [`xcb_request_check`].')
        _h('///')
        _h('/// [`discard_reply`]: crate::Xcb::xcb_discard_reply')
        _h('/// [`xcb_request_check`]: crate::Xcb::xcb_request_check')
    if self.c_need_aux and not aux:
        _h('///')
        _h(f'/// There is an auxiliary version of this function: [`{aux_name}`].')
        _h('///')
        _h(f'/// [`{aux_name}`]: Self::{aux_name}')

    params = [('c', '*mut xcb_connection_t')]
    for field in param_fields:
        if field.type.c_need_serialize and not aux:
            ty = f'*const c_void'
        elif field.has_c_pointer:
            if field.has_c_field_const_type:
                ty = f'*const {field.c_field_type}'
            else:
                ty = f'*mut {field.c_field_type}'
        else:
            ty = field.c_field_type
        params.append((to_snake_case(field.c_field_name), ty))

    add_sym(func_name, params, rv=func_cookie)


def _c_reply(self, name):
    '''
    Declares the function that returns the reply structure.
    '''

    _h_setlevel(2)
    _h('')
    _h(f'/// Waits for the reply to a `{_opcode_name(name)}` request.')
    params = [
        ('c', '*mut xcb_connection_t'),
        ('cookie', self.c_cookie_type),
        ('e', '*mut *mut xcb_generic_error_t'),
    ]
    add_sym(self.c_reply_name, params, rv=f'*mut {self.c_reply_type}')


def _c_reply_has_fds(self):
    return any(field.isfd for field in self.fields)


def _c_reply_fds(self, name):
    '''
    Declares the function that returns fds related to the reply.
    '''
    _h_setlevel(2)
    _h('')
    _h(f'/// Retrieves the file descriptors from the reply to a `{_opcode_name(name)}` request.')
    _h(f'///')
    _h(f'/// The returned pointer must be freed with `libc::free`.')
    params = [
        ('c', '*mut xcb_connection_t'),
        ('reply', f'*mut {self.c_reply_type}'),
    ]
    add_sym(self.c_reply_fds_name, params, rv='*mut c_int')


def _opcode_name(name):
    opcode_name = name[-1]
    if _ns.is_ext:
        opcode_name = f'{_ns.ext_name}::{opcode_name}'
    return opcode_name


def _c_opcode(name, opcode, kind, container, is_ge_event=False):
    '''
    Declares the opcode define for requests, events, and errors.
    '''
    ty = 'u8'
    opcode_name = _opcode_name(name)
    _h_setlevel(0)
    _h('')
    _h(f'/// The opcode for `{opcode_name}` {kind}.')
    _h(f'///')
    if kind == 'requests':
        ext = '[`ptr::null_mut()`](std::ptr::null_mut())'
        if _ns.is_ext:
            ext = f'[`{_ns.rust_obj_name}::{_ns.c_ext_global_name}()`]'
        _h(f'/// If this value appears in [`xcb_protocol_request_t::opcode`], and')
        _h(f'/// [`xcb_protocol_request_t::ext`] is {ext}, then the type of the request is')
        _h(f'/// [`{container}`].')
    elif kind == 'errors':
        if _ns.is_ext:
            _h(f'/// If this value plus the extension error base appears in [`xcb_generic_error_t::error_code`],')
            _h(f'/// then the type of the error is [`{container}`].')
        else:
            _h(f'/// If this value appears in [`xcb_generic_error_t::error_code`], then the type of the')
            _h(f'/// error is [`{container}`].')
    elif kind == 'events':
        if is_ge_event:
            if not _ns.is_ext:
                raise
            ty = 'u16'
            _h(f'/// If this value appears in [`xcb_ge_generic_event_t::event_type`], and')
            _h(f'/// [`xcb_ge_generic_event_t::extension`] is the opcode of the `{_ns.ext_name}` extension,')
            _h(f'/// then the type of the event is [`{container}`].')
        elif _ns.is_ext:
            _h(f'/// If this value plus the extension event base appears in [`xcb_generic_event_t::response_type`],')
            _h(f'/// then the type of the event is [`{container}`].')
        else:
            _h(f'/// If this value appears in [`xcb_generic_event_t::response_type`], then the type of the')
            _h(f'/// event is [`{container}`].')
    else:
        raise
    _h(f'pub const {_n(name).upper()}: {ty} = {opcode}i32 as {ty};')


def _c_cookie(self, name):
    '''
    Declares the cookie type for a non-void request.
    '''
    _h_setlevel(0)
    _h('')
    _h(f'/// The cookie for the reply to a `{_opcode_name(name)}` request.')
    _h(f'///')
    _h(f'/// Pass this cookie to [`{self.c_reply_name}`] to retrieve the reply.')
    _h(f'///')
    _h(f'/// [`{self.c_reply_name}`]: {_ns.rust_obj_name}::{self.c_reply_name}')
    _h('#[derive(Copy, Clone, Debug)]')
    _h('#[repr(C)]')
    _h('pub struct %s {', self.c_cookie_type)
    _h('    /// The sequence number of the request.')
    _h('    pub sequence: c_uint,')
    _h('}')
    impl_default(self.c_cookie_type)


def c_request(self, name):
    '''
    Exported function that handles request declarations.
    '''
    _c_type_setup(self, name, ('request',))

    if self.reply:
        # Cookie type declaration
        _c_cookie(self, name)

    # Opcode define
    _c_opcode(name, self.opcode, 'requests', self.c_type)

    # Request structure declaration
    _c_complex(self, _opcode_name(name), 'request')

    if self.reply:
        _c_type_setup(self.reply, name, ('reply',))
        # Request prototypes
        has_fds = _c_reply_has_fds(self.reply)
        # Reply structure definition
        _c_complex(self.reply, _opcode_name(name), 'reply', has_fds=has_fds)
        _c_request_helper(self, name, void=False, regular=True, aux=False)
        _c_request_helper(self, name, void=False, regular=False, aux=False)
        if self.c_need_aux:
            _c_request_helper(self, name, void=False, regular=True, aux=True)
            _c_request_helper(self, name, void=False, regular=False, aux=True)
        # Reply accessors
        _c_accessors(self.reply, name + ('reply',), name)
        _c_reply(self, name)
        if has_fds:
            _c_reply_fds(self, name)
    else:
        # Request prototypes
        _c_request_helper(self, name, void=True, regular=False)
        _c_request_helper(self, name, void=True, regular=True)
        if self.c_need_aux:
            _c_request_helper(self, name, void=True, regular=False, aux=True)
            _c_request_helper(self, name, void=True, regular=True, aux=True)
        for field in self.fields:
            if not field.type.is_pad and field.wire:
                if _c_field_needs_list_accessor(field):
                    _c_accessors_list(self, field)
                elif _c_field_needs_field_accessor(field):
                    _c_accessors_field(self, field)


def c_eventstruct(self, name):
    # add fields that are needed to get the event-type in a generic way
    self.fields.append(Field(tevent, tevent.name, 'event_header', False, True, True))

    if self.contains_ge_events:
        # TODO: add header of ge-events as an extra field
        raise Exception('eventstructs with ge-events are not yet supported')

    _c_type_setup(self, name, ())

    # correct the format of the field names
    for field in self.fields:
        field.c_field_name = _n_item(field.c_field_name).lower()

    _c_complex(self, _opcode_name(name), 'eventstruct')
    _c_iterator(self, name)

    if not self.fixed_size():
        # TODO: Create sizeof function (and maybe other accessors) for var-sized eventstructs
        raise Exception('var sized eventstructs are not yet supported')


def c_event(self, name):
    '''
    Exported function that handles event declarations.
    '''

    # The generic event structure xcb_ge_event_t has the full_sequence field
    # at the 32byte boundary. That's why we've to inject this field into GE
    # events while generating the structure for them. Otherwise we would read
    # garbage (the internal full_sequence) when accessing normal event fields
    # there.
    force_packed = False
    if hasattr(self, 'is_ge_event') and self.is_ge_event and self.name == name:
        event_size = 0
        for field in self.fields:
            if field.type.size != None and field.type.nmemb != None:
                event_size += field.type.size * field.type.nmemb
            if event_size == 32:
                full_sequence = Field(tcard32, tcard32.name, 'full_sequence', False, True, True)
                idx = self.fields.index(field)
                self.fields.insert(idx + 1, full_sequence)

                # If the event contains any 64-bit extended fields, they need
                # to remain aligned on a 64-bit boundary.  Adding full_sequence
                # would normally break that; force the struct to be packed.
                force_packed = any(f.type.size == 8 and f.type.is_simple for f in self.fields[(idx + 1):])
                break

    if self.name == name:
        _c_type_setup(self, name, ('event',))
        # generate accessors
        # (needed for fields after var-sized fields, for lists with var-sized elements,
        # switches, ...)
        _c_accessors(self, name, name)
    else:
        # no type-setup needed for eventcopies
        # (the type-setup of an eventcopy would overwrite members of the original
        # event, and it would create sizeof-etc funtions which
        # called undefined accessor functions)
        pass

    is_ge_event = hasattr(self, 'is_ge_event') and self.is_ge_event and name[-1] != 'GeGeneric'
    if self.name == name:
        c_type = self.c_type
    else:
        c_type = _t(name + ("event",))

    # Opcode define
    _c_opcode(name, self.opcodes[name], 'events', c_type, is_ge_event=is_ge_event)

    opcode_name = _opcode_name(name)
    if self.name == name:
        # Structure definition
        _c_complex(self, opcode_name, 'event', force_packed=force_packed)
    else:
        # Typedef
        _h('')
        _h(f'/// The `{opcode_name}` event.')
        _h(f'pub type {c_type} = {_t(self.name + ("event",))};')

        # Create sizeof-function for eventcopies for compatibility reasons
        if self.c_need_sizeof:
            _h_setlevel(2)
            _h('')
            _h(f'/// Computes the size of a `{c_type}` object.')
            _h('///')
            _h('/// Note: The libxcb function uses `const void*` as an argument because all pointers')
            _h('/// implicitly coerce to `const void*`. This is not the case in Rust so we have to use')
            _h('/// the correct pointer type to ensure backwards compatibility.')
            add_sym(_n(name + ('sizeof',)), [('buffer', f'*const {c_type}')], rv='c_int')


def c_error(self, name):
    '''
    Exported function that handles error declarations.
    '''
    _c_type_setup(self, name, ('error',))

    if self.name == name:
        c_type = self.c_type
    else:
        c_type = _t(name + ("error",))

    # Opcode define
    _c_opcode(name, self.opcodes[name], 'errors', c_type)

    opcode_name = _opcode_name(name)
    if self.name == name:
        # Structure definition
        _c_complex(self, opcode_name, 'error')
    else:
        # Typedef
        _h_setlevel(0)
        _h('')
        _h(f'/// The `{opcode_name}` error.')
        _h(f'pub type {c_type} = {_t(self.name + ("error",))};')


# Main routine starts here

# Must create an "output" dictionary before any xcbgen imports.
output = {'open': c_open,
          'close': c_close,
          'simple': c_simple,
          'enum': c_enum,
          'struct': c_struct,
          'union': c_union,
          'request': c_request,
          'eventstruct': c_eventstruct,
          'event': c_event,
          'error': c_error,
          }

# Boilerplate below this point

# Import the module class
try:
    from xcbgen.state import Module
    from xcbgen.xtypes import *
except ImportError:
    print('''
Failed to load the xcbgen Python package!
Make sure that xcb/proto installed it on your Python path.
If not, you will need to create a .pth file or define $PYTHONPATH
to extend the path.
Refer to the README file in xcb/proto for more info.
''')
    raise

# predefined datatype globals.
tevent = SimpleType(('xcb_raw_generic_event_t',), 32)

# Parse the xml header
module = Module(sys.argv[1], output)

# Build type-registry and resolve type dependencies
module.register()
module.resolve()

# Output the code
module.generate()
