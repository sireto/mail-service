use serde_json::Value;

pub fn secure_server_response(mut creds: Value) -> Value {
    if let Value::Object(map) = &mut creds {
        map.insert("access_key_id".into(), Value::String("*************".to_string()));
        map.insert("secret_access_key".into(), Value::String("*************".to_string()));
    }
    creds
}