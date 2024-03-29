use rand;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(value: i32) -> i32 {
    value +1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_add_one() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}
