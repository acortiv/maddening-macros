#[macro_export]
macro_rules! mm_assert_ne {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!(
                        "assertion failed: `(left != right)`\n left: `{:?}`\n right: `{:?}`",
                        left_val,
                        right_val
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    panic!($($args)+)
                }
            }
        }
    };
}
