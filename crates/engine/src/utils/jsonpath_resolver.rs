use jsonpath_rust::JsonPath;
use ryvus_core::prelude::ExecutionContext;
use serde_json::{json, Value};

/// Builds a JSON structure for JSONPath resolution with:
/// - $.payload
/// - $.<step>.output.<field>
pub fn build_jsonpath_context(ctx: &ExecutionContext) -> Value {
    let mut map = serde_json::Map::new();

    // 1️⃣ Include the payload from context.data
    if let Some(payload) = ctx.data.get("payload") {
        map.insert("payload".into(), payload.clone());
    }

    // 2️⃣ Include all step outputs keyed by step.key
    for step in &ctx.steps {
        if let Some(output) = &step.output {
            map.insert(step.key.clone(), json!({ "output": output }));
        }
    }

    // 3️⃣ Add a shortcut to the most recent output
    if let Some(last_step) = ctx.steps.last() {
        if let Some(output) = &last_step.output {
            map.insert("output".into(), output.clone());
        }
    }

    Value::Object(map)
}

/// Recursively resolve any JSONPath or secret:$. references
pub fn resolve_jsonpaths(value: &mut Value, ctx_json: &Value) {
    match value {
        Value::Object(map) => {
            for v in map.values_mut() {
                resolve_jsonpaths(v, ctx_json);
            }
        }
        Value::Array(arr) => {
            for v in arr.iter_mut() {
                resolve_jsonpaths(v, ctx_json);
            }
        }
        Value::String(s) if s.starts_with("$.") || s.starts_with("secret:$.") => {
            let expr = if s.starts_with("secret:") {
                s.replacen("secret:", "", 1)
            } else {
                s.clone()
            };

            // perform JSONPath query
            match ctx_json.query_with_path(expr.as_str()) {
                Ok(results) if !results.is_empty() => {
                    let first = results.first().unwrap().clone();
                    let inner_val = first.val();
                    *value = inner_val.clone();
                }
                Ok(_) => {
                    // nothing found, leave as-is
                }
                Err(err) => {
                    eprintln!("JsonPath resolution error for '{}': {}", expr, err);
                }
            }
        }
        _ => {}
    }
}
