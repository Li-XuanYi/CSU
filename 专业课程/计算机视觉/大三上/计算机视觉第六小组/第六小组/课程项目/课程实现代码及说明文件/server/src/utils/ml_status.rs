use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;

pub fn merge_ml_status(existing: Option<JsonValue>, updates: &[(&str, &str)]) -> JsonValue {
    let mut map = match existing {
        Some(JsonValue::Object(obj)) => obj,
        _ => JsonMap::new(),
    };
    for (key, value) in updates {
        map.insert(key.to_string(), JsonValue::String(value.to_string()));
    }
    JsonValue::Object(map)
}

pub fn build_ml_status(tasks: &[String]) -> JsonValue {
    let mut map = JsonMap::new();
    for task in tasks {
        map.insert(task.clone(), JsonValue::String("pending".to_string()));
    }
    JsonValue::Object(map)
}

pub fn get_task_status(status: &Option<JsonValue>, task: &str) -> Option<String> {
    status
        .as_ref()
        .and_then(|val| val.get(task))
        .and_then(|val| val.as_str())
        .map(|s| s.to_string())
}
