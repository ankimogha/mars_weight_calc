#[cfg(test)]
mod test {
    use crate::converter::weight_converter::{calculate_weight_on_mars, convert_weight};

    #[test]
    fn check_calculate_weight_on_mars() {
        assert_eq!(calculate_weight_on_mars(100.0), 37.828747);
    }

    #[test]
    fn check_convert_weight() {
        assert_eq!(convert_weight("100.0".to_string()),"100.0".to_string());
    }

    #[test]
    fn check_input_parsing_fails() {
        assert_eq!(convert_weight(" ".to_string()),"cannot parse float from empty string".to_string());
    }
}