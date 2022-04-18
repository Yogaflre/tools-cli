include!("../src/time/mod.rs");

#[cfg(test)]
mod format {
    use crate::formatter;

    #[test]
    fn now() {
        formatter::now();
    }

    #[test]
    fn convert() {
        assert_eq!(
            formatter::convert("1649947507679"),
            format!(
                "{}\n{}\n{}\n{}",
                "1649947507", "1649947507679", "2022-04-14 22:45:07", "2022-04-14T22:45:07Z"
            )
        );
    }
}
