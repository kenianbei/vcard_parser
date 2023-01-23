#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use vcard_parser::parse_vcards;

    #[test]
    fn concat() {
        assert!(parse_vcards(read_to_string("tests/assets/concat.vcf").unwrap().as_str()).is_ok());
    }

    #[test]
    fn multiple() {
        assert!(parse_vcards(read_to_string("tests/assets/multiple.vcf").unwrap().as_str()).is_ok());
    }

    #[test]
    fn photo() {
        assert!(parse_vcards(read_to_string("tests/assets/photo.vcf").unwrap().as_str()).is_ok());
    }

    #[test]
    fn single() {
        assert!(parse_vcards(read_to_string("tests/assets/single.vcf").unwrap().as_str()).is_ok());
    }
}
