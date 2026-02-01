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

pub trait FoldToDisplay {
    fn fold_to_display(&self) -> String;
}

#[macro_export]
macro_rules! fold_to_display_for {
    ($last:ident) => {
        impl<$last: Display> FoldToDisplay for ($last,) {
            fn fold_to_display(&self) -> String {
                let (t,) = self;
                t.to_string()
            }
        }
    };
    ($first:ident , $($rest:ident),+) => {
        $crate::fold_to_display_for!($($rest),+);

        impl<$first, $($rest,)* D: Display> FoldToDisplay for ($first, $($rest,)* D ) {
            fn fold_to_display(&self) -> String {
                let ($first, $($rest,)* t) = self;
                let mut acc = String::new();
                acc.push_str(&$first.to_string());
                $(
                    acc.push_str(&$rest.to_string());
                )*
                acc.push_str(&t.to_string());
                acc
        }
        }
    };
}
