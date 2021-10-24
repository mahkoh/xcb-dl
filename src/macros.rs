macro_rules! call {
    ($self:expr, $f:ident) => {
        ($self.$f.get(&$self.lib, concat!(stringify!($f), "\0")))
    };
}
