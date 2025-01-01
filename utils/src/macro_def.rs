/// 检查两个浮点数是否相等
#[macro_export]
macro_rules! check_f64_eq {
    ($left:expr, $right:expr $(,)?) => {
        ($left as f64 - $right as f64).abs() < f64::EPSILON
    };
}

/// 断言两个浮点数相等
#[macro_export]
macro_rules! assert_f64_eq {
    ($left:expr, $right:expr $(,)?) => {
        if !($crate::check_f64_eq!($left, $right)) {
            panic!("Expected `left`: {:?}, get `right`: {:?}", $left, $right);
        }
    };
}

/// 断言两个浮点数不相等
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
    #[test]
    fn test_macro() {
        assert_f64_eq!(2.33444, 2.33444);
        assert_f64_ne!(2.33444, 2.33445);

        assert_f64_eq!(3, 3.);
        assert_f64_ne!(3, 3.1);
    }
}
