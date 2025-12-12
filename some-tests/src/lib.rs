mod operations;

// TODO to check
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn get_address(s: &String) -> String {
    // it returns the address of the string
    format!("{:p}", s)
}

pub fn get_address_i32(s: &i32) -> String {
    format!("{:p}", s)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_type_of_using_super() {
        use super::type_of;

        let s = String::from("omar");
        assert_eq!(type_of(&s), "alloc::string::String");
    }

    #[test]
    fn test_type_of_using_crate() {
        use crate::type_of;

        let s = String::from("omar");
        assert_eq!(type_of(&s), "alloc::string::String");
    }

    #[test]
    fn test_add() {
        use super::add;

        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sub() {
        use crate::operations::sub;

        assert_eq!(sub(4, 1), 3);
    }
}
