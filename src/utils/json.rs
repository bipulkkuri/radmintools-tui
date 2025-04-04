use serde_json::{to_string_pretty, Value};

pub fn pretty_json_from_string(json_str: &str) -> Result<String, serde_json::Error> {
    let parsed_json: Value = serde_json::from_str(json_str)?;
    to_string_pretty(&parsed_json)
}
