include!("../src/parser/mod.rs");

#[cfg(test)]
mod json_test {
    use crate::json;
    const ORIGINAL: &str = r#"{"age":1,"name":"tools"}"#;
    const PRETTY: &str = r#"{
  "age": 1,
  "name": "tools"
}"#;

    #[test]
    fn format() {
        assert_eq!(json::format(ORIGINAL), PRETTY);
    }

    #[test]
    fn compress() {
        assert_eq!(json::compress(PRETTY), ORIGINAL);
    }
}
