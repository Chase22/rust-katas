#[cfg(test)]
mod tests {
    use crate::fizz_buzz;

    #[test]
    fn test_number() {
        assert_eq!(fizz_buzz(13), 13.to_string());
    }

    #[test]
    fn test_fizz() {
        assert_eq!(fizz_buzz(3), String::from("fizz"));
    }

    #[test]
    fn test_buzz() {
        assert_eq!(fizz_buzz(5), String::from("buzz"));
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizz_buzz(15), String::from("fizzbuzz"));
    }
}