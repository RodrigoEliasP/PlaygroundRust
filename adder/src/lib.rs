pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn it_should_panic() {
        panic!();
    }
    #[test]
    fn test_with_result() -> Result<(), String> {
        Err("Error testing".to_owned())
    }
}
