pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn virables() {
        // it is immutalbe, so cannot assign twice to immutable variable
        let x = 4;
        assert_eq!(x, 4);

        // with mut, it is mutable
        let mut y = 4;
        assert_eq!(y, 4);
        y = 10;
        assert_eq!(y, 10);

        // rust allows to overrides variables
        let z = 4;
        assert_eq!(z, 4);
        let z = 20;
        assert_eq!(z, 20);

        // rust allows to overrides variables changing the type
        let a = 4;
        assert_eq!(a, 4);
        let a = "omar";
        assert_eq!(a, "omar");
    }

    #[test]
    fn virables_shadowing() {
        let x = 4;
        assert_eq!(x, 4);

        {
            // interior scope, this x valuse only lives here
            let x = x - 2;
            assert_eq!(x, 2);
        }

        let x = x + 1;
        assert_eq!(x, 5);
    }

    #[test]
    fn constant() {
        // compiler forces to be UPERCASE and define a type
        // a const cannot be override or redifine
        const X: u8 = 4;
        assert_eq!(X, 4);
    }
}
