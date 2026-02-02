use std::fmt::Display;
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

// Pattern for defining trait implementation for variable length tuple
pub trait ExampleVariableLengthTupleTrait {}

// Macro Export is not needed as this is done entirely internally
// #[macro_export]
macro_rules! example_variable_length_tuple_trait_impl {
    ( $($component:ident),* ; L ) => {
        impl<L: Display, $($component),*> ExampleVariableLengthTupleTrait for ($($component,)* L,) {}
    };
}

example_variable_length_tuple_trait_impl!(; L);
example_variable_length_tuple_trait_impl!(T1 ; L);
example_variable_length_tuple_trait_impl!(T1, T2 ; L);
example_variable_length_tuple_trait_impl!(T1, T2, T3 ; L);
example_variable_length_tuple_trait_impl!(T1, T2, T3, T4 ; L);
example_variable_length_tuple_trait_impl!(T1, T2, T3, T4, T5 ; L);
