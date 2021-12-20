use proc_macro;

// Simplified implementation of `vec!` macro
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ), *) => {
        {
            let mut tmp  = Vec::new();
            $(
                tmp.push($x);
            )*
            tmp
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let mut want: Vec<i32> = Vec::new();
        want.push(1);
        want.push(2);

        let got: Vec<i32> = vec![1, 2];
        assert_eq!(want, got);
    }
}
