#[macro_export]
macro_rules! check_f64_eq {
    ($left:expr, $right:expr $(,)?) => {
        ($left as f64 - $right as f64).abs() < f64::EPSILON
    };
}

#[macro_export]
macro_rules! assert_f64_eq {
    ($left:expr, $right:expr $(,)?) => {
        if !($crate::check_f64_eq!($left, $right)) {
            panic!("Expected `left`: {:?}, get `right`: {:?}", $left, $right);
        }
    };
}

#[macro_export]
macro_rules! assert_f64_ne {
    ($left:expr, $right:expr $(,)?) => {
        if $crate::check_f64_eq!($left, $right) {
            panic!("`left` == `right` == {:?}", $left);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_f64_eq!(2.33444, 2.33444);
        assert_f64_ne!(2.33444, 2.33445);

        assert_f64_eq!(3, 3.);
        assert_f64_ne!(3, 3.1);
    }
}
