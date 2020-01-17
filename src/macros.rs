// A helper macro for joining multiple strings together
#[macro_export]
macro_rules! push_own {
    // catch self.var pattern
    ($s:ident . $inner:ident, $( $push_val:expr),+) => {
        {
            let save = &$s.$inner;
            push_own!{save, $($push_val),+}
        }
    };
    // catch var pattern (no "self.")
    ($s:ident, $( $push_val:expr),+) => {
        {
            let mut save = $s.clone().to_string();
            $(
                save.push_str($push_val);
            )+
            save
        }
    };
}