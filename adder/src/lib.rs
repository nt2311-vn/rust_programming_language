pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
