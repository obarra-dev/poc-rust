mod operations;

#[cfg(test)]
mod tests {
    use crate::operations::sub;

    #[test]
    fn test_something_simple() {
        assert!(true);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(4, 1), 3);
    }
}
