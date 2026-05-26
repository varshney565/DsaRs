pub mod level1;
pub mod helper;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_print_1d() {
        let v = vec![1,2,3,4,5,6,7];
        helper::display1d(&v);
    }

    #[test]
    pub fn test_print_2d() {
        let v = vec![vec![1,2,3],vec![4,5,6]];
        helper::display2d(&v);
    }

    #[test]
    pub fn test_fibonacci_recursive() {
        assert_eq!(level1::fib::fibonacci_recursive(5),5);
        assert_eq!(level1::fib::fibonacci_recursive(6),8);
        assert_eq!(level1::fib::fibonacci_recursive(7),13);
    }

    #[test]     
    pub fn test_fibonacci_memo() {
        let mut dp = vec![-1;10];
        assert_eq!(level1::fib::fibonacci_memo(5,&mut dp),5);
        dp = vec![-1;10];
        assert_eq!(level1::fib::fibonacci_memo(6,&mut dp),8);
        dp = vec![-1;10];
        assert_eq!(level1::fib::fibonacci_memo(7,&mut dp),13);
    }

}