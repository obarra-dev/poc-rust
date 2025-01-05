pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// TODO to check
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
