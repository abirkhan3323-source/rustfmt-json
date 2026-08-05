// rustfmt-json library — reusable JSON formatting and validation functions.

use serde_json::{Map, Value};
use std::io::{self, Read, Write};

/// Format JSON with indentation. Returns formatted string or error.
pub fn pretty_print(input: &str) -> Result<String, serde_json::Error> {
    let value: Value = serde_json::from_str(input)?;
    Ok(serde_json::to_string_pretty(&value).unwrap_or_default())
}

/// Minify JSON by removing all whitespace.
pub fn minify(input: &str) -> Result<String, serde_json::Error> {
    let value: Value = serde_json::from_str(input)?;
    Ok(serde_json::to_string(&value).unwrap_or_default())
}

/// Validate JSON and return detailed error or Ok(()).
pub fn validate(input: &str) -> Result<(), serde_json::Error> {
    serde_json::from_str::<Value>(input).map(|_| ())
}

/// Sort object keys recursively. Useful for diffing JSON files.
pub fn sort_keys(input: &str) -> Result<String, serde_json::Error> {
    let value: Value = serde_json::from_str(input)?;
    let sorted = sort_value(value);
    Ok(serde_json::to_string_pretty(&sorted).unwrap_or_default())
}

fn sort_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut entries: Vec<_> = map.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            let sorted_map: Map<String, Value> = entries
                .into_iter()
                .map(|(k, v)| (k, sort_value(v)))
                .collect();
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(sort_value).collect())
        }
        other => other,
    }
}

/// Extract a value by dot-separated path. E.g., `get("a.b.0.c", json)`.
pub fn get<'a>(path: &str, value: &'a Value) -> Option<&'a Value> {
    let mut current = value;
    for key in path.split('.') {
        match current {
            Value::Object(map) => current = map.get(key)?,
            Value::Array(arr) => {
                let idx: usize = key.parse().ok()?;
                current = arr.get(idx)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Convert JSON to YAML (basic, no external deps).
pub fn to_yaml(input: &str) -> Result<String, String> {
    let value: Value = serde_json::from_str(input).map_err(|e| e.to_string())?;
    let mut output = String::new();
    value_to_yaml(&value, 0, &mut output);
    Ok(output)
}

fn value_to_yaml(value: &Value, indent: usize, output: &mut String) {
    let prefix = "  ".repeat(indent);
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                output.push_str(&format!("{}{}:\n", prefix, k));
                value_to_yaml(v, indent + 1, output);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                output.push_str(&format!("{}- ", prefix));
                if let Value::Object(_) = item {
                    output.push('\n');
                    value_to_yaml(item, indent + 1, output);
                } else if let Value::Array(_) = item {
                    output.push('\n');
                    value_to_yaml(item, indent + 1, output);
                } else {
                    output.push_str(&format_value(item));
                    output.push('\n');
                }
            }
        }
        other => {
            output.push_str(&format_value(other));
            output.push('\n');
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            if s.contains('\n') || s.contains('"') || s.contains(':') {
                format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            } else {
                s.clone()
            }
        }
        Value::Array(_) | Value::Object(_) => String::new(),
    }
}

/// Read all input from stdin and write to stdout.
pub fn process_stdin<F>(f: F) -> i32
where
    F: Fn(&str) -> Result<String, String>,
{
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error: failed to read stdin");
        return 1;
    }
    match f(&input) {
        Ok(output) => {
            let _ = io::stdout().write_all(output.as_bytes());
            0
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid() {
        assert!(validate(r#"{"a": 1}"#).is_ok());
    }

    #[test]
    fn test_validate_invalid() {
        assert!(validate(r#"{"a": 1"#).is_err());
    }

    #[test]
    fn test_minify() {
        let input = r#"{
            "a": 1,
            "b": 2
        }"#;
        let result = minify(input).unwrap();
        assert!(!result.contains('\n'));
    }

    #[test]
    fn test_sort_keys() {
        let input = r#"{"b": 2, "a": 1}"#;
        let result = sort_keys(input).unwrap();
        assert!(result.find("\"a\"").unwrap() < result.find("\"b\"").unwrap());
    }

    #[test]
    fn test_get_nested() {
        let json: Value = serde_json::from_str(r#"{"a": {"b": [10, 20, 30]}}"#).unwrap();
        assert_eq!(get("a.b.1", &json).unwrap().as_i64().unwrap(), 20);
    }
}
