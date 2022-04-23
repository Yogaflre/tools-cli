use serde_json::{Map, Value};

pub fn format(json: &str) -> String {
    return serde_json::to_string_pretty(&parse_json(json)).unwrap();
}

pub fn compress(json: &str) -> String {
    return serde_json::to_string(&parse_json(json)).unwrap();
}

fn parse_json(json: &str) -> Map<String, Value> {
    return serde_json::from_str(json).unwrap();
}
