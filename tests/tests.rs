mod tests {
    #[test]
    fn new_store_contains_no_data() {
        let s = rsk::Store::new();
        let empty = Vec::new();
        assert_eq!(s.all(), empty)
    }
}
