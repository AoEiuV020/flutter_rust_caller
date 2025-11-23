use crate::business::*;
use serde_json::{json, Value};

/// Call 根据方法名和JSON参数字符串调用对应函数（同步版本）
/// method: 要调用的方法名
/// paramJSON: 包含参数的JSON字符串
/// 返回: 包含执行结果的JSON字符串
pub fn call(method: &str, param_json: &str) -> String {
    // 解析JSON参数为map
    let params = match parse_params(param_json) {
        Ok(Value::Object(map)) => Value::Object(map),
        Ok(_) => return error_response("Parameters must be a JSON object"),
        Err(e) => return error_response(&e),
    };

    // 调用Execute函数执行具体逻辑
    execute(method, &params)
}

/// Call 异步版本，用于 WASM 中需要异步执行的调用
#[cfg(target_arch = "wasm32")]
pub async fn call_async(method: &str, param_json: &str) -> String {
    // 解析JSON参数为map
    let params = match parse_params(param_json) {
        Ok(Value::Object(map)) => Value::Object(map),
        Ok(_) => return error_response("Parameters must be a JSON object"),
        Err(e) => return error_response(&e),
    };

    // 调用异步Execute函数执行具体逻辑
    execute_async(method, &params).await
}

/// Execute 执行具体函数调用并返回结果 JSON
/// 通用的同步执行逻辑，对所有方法统一处理，无特殊逻辑
pub fn execute(method: &str, params: &Value) -> String {
    let result = match method {
        "Increase" => {
            json!({ "result": increase() })
        }
        "Sum" => {
            // 提取参数
            let a = match extract_int_param(params, "a") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let b = match extract_int_param(params, "b") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            json!({ "result": sum(a, b) })
        }
        "SumLongRunning" => {
            // 提取参数
            let a = match extract_int_param(params, "a") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let b = match extract_int_param(params, "b") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            json!({ "result": sum_long_running(a, b) })
        }
        _ => {
            return error_response(&format!("Unknown method: {}", method));
        }
    };

    result.to_string()
}

/// 异步Execute版本（仅在 WASM 中可用）
/// 通用的异步执行逻辑，对所有方法统一处理，无特殊逻辑
/// 使用异步函数版本来支持需要异步操作的函数（如 sum_long_running_async）
#[cfg(target_arch = "wasm32")]
pub async fn execute_async(method: &str, params: &Value) -> String {
    let result = match method {
        "Increase" => {
            json!({ "result": increase() })
        }
        "Sum" => {
            // 提取参数
            let a = match extract_int_param(params, "a") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let b = match extract_int_param(params, "b") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            json!({ "result": sum(a, b) })
        }
        "SumLongRunning" => {
            // 提取参数
            let a = match extract_int_param(params, "a") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let b = match extract_int_param(params, "b") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            // 在 WASM 中，使用异步版本的 sum_long_running 以支持异步 sleep
            json!({ "result": sum_long_running_async(a, b).await })
        }
        _ => {
            return error_response(&format!("Unknown method: {}", method));
        }
    };

    result.to_string()
}

/// 将 JSON 字符串解析为 Value
fn parse_params(param_json: &str) -> Result<Value, String> {
    serde_json::from_str(param_json)
        .map_err(|e| format!("Failed to parse parameters: {}", e))
}

/// 从参数中提取整数参数
fn extract_int_param(params: &Value, key: &str) -> Result<i32, String> {
    match params.get(key) {
        Some(Value::Number(n)) => match n.as_i64() {
            Some(v) => Ok(v as i32),
            None => Err(format!("Parameter {} must be an integer", key)),
        },
        _ => Err(format!("Missing or invalid parameter {}", key)),
    }
}

/// 创建错误响应 JSON
fn error_response(error: &str) -> String {
    json!({ "error": error }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_sum() {
        let result = call("Sum", r#"{"a": 5, "b": 3}"#);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["result"], 8);
    }

    #[test]
    fn test_call_increase() {
        let result = call("Increase", "{}");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["result"].is_number());
    }

    #[test]
    fn test_call_unknown_method() {
        let result = call("UnknownMethod", "{}");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["error"].is_string());
    }

    #[test]
    fn test_call_invalid_json() {
        let result = call("Sum", "invalid json");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["error"].is_string());
    }

    #[test]
    fn test_call_missing_param() {
        let result = call("Sum", r#"{"a": 5}"#);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["error"].is_string());
    }
}
