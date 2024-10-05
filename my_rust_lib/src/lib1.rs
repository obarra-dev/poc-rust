pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 38);
        assert_eq!(result, 40);
    }
    
    #[test]
    fn it_works_assuredly() {
        let result = add(2, 38);
        assert_ne!(result, 44);
    }
}
