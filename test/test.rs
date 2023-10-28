#[cfg(test)]
mod tests {
    use count_num_cli::{logic, string_to_list};
    
    #[test]
    fn test_logic() {
        let text = "1, 2, 3";
        let numbers = string_to_list(text);
        assert_eq!(numbers, vec![1, 2, 3]);
        let map_ = logic(numbers);
        // assert_eq!(text, decrypted);
    }
}