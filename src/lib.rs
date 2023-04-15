pub fn bla() {}

#[cfg(test)]
mod tests {
    use crate::bla;

    #[test]
    fn unit_test_works() {
        bla();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
