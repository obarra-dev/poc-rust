pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use crate::operations::sub;

    #[test]
    fn test_sub() {
        assert_eq!(sub(4, 1), 3);
    }
}
