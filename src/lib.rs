pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(4), 6);
    }

    #[test]
    fn not_working() {
        assert_eq!(add_two(4), 7);
    }
}
