use crate::business::*;
use serde_json::{json, Value};

/// Call 根据方法名和JSON参数字符串调用对应函数
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

/// CallAsync 异步版本，供 WASM 使用
/// 对于异步操作，可以在此处实现异步逻辑
pub async fn call_async(method: &str, param_json: &str) -> String {
    // 解析JSON参数为map
    let params = match parse_params(param_json) {
        Ok(Value::Object(map)) => Value::Object(map),
        Ok(_) => return error_response("Parameters must be a JSON object"),
        Err(e) => return error_response(&e),
    };

    // 调用ExecuteAsync函数执行具体逻辑
    execute_async(method, &params).await
}

/// Execute 执行具体函数调用并返回结果 JSON
/// 通用的同步执行逻辑，对所有方法统一处理
pub fn execute(method: &str, params: &Value) -> String {
    let result = match method {
        "Multiply" => {
            // 提取参数：base, multiplier
            let base = match extract_int_param(params, "base") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let multiplier = match extract_int_param(params, "multiplier") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            json!({ "result": multiply(base, multiplier) })
        }
        "PowerMultiply" => {
            // 提取参数：base, exponent_base, exponent
            let base = match extract_int_param(params, "base") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let exponent_base = match extract_int_param(params, "exponent_base") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let exponent = match extract_int_param(params, "exponent") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            json!({ "result": power_multiply(base, exponent_base, exponent) })
        }
        "NextPrime" => {
            // 提取参数
            let base = match extract_int_param(params, "base") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            let count = match extract_int_param(params, "count") {
                Ok(v) => v,
                Err(e) => return error_response(&e),
            };

            json!({ "result": next_prime(base, count) })
        }
        "GetLastPrime" => {
            json!({ "result": get_last_prime() })
        }
        _ => {
            return error_response(&format!("Unknown method: {}", method));
        }
    };

    result.to_string()
}

/// ExecuteAsync 异步执行函数调用
/// 基于同步版本，可以支持需要异步的方法
pub async fn execute_async(method: &str, params: &Value) -> String {
    // 当前所有方法都可以同步执行，所以直接调用同步版本
    // 如果未来需要真正的异步方法，可以在此处添加特殊处理
    execute(method, params)
}

/// 将 JSON 字符串解析为 Value
fn parse_params(param_json: &str) -> Result<Value, String> {
    serde_json::from_str(param_json)
        .map_err(|e| format!("Failed to parse parameters: {}", e))
}

/// 从参数中提取整数参数
fn extract_int_param(params: &Value, key: &str) -> Result<i64, String> {
    match params.get(key) {
        Some(Value::Number(n)) => match n.as_i64() {
            Some(v) => Ok(v),
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
    fn test_call_multiply() {
        let result = call("Multiply", r#"{"base": 5, "multiplier": 10}"#);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["result"], 50);
    }

    #[test]
    fn test_call_power_multiply() {
        let result = call("PowerMultiply", r#"{"base": 10, "exponent_base": 10, "exponent": 10}"#);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["result"], 100000000000i64);
    }

    #[test]
    fn test_call_next_prime() {
        let result = call("NextPrime", r#"{"base": 10, "count": 1}"#);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["result"], 11);
    }

    #[test]
    fn test_call_get_last_prime() {
        // 先调用 NextPrime 以设置值
        call("NextPrime", r#"{"base": 20, "count": 1}"#);
        
        // 然后获取保存的值
        let result = call("GetLastPrime", "{}");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["result"], 23);
    }

    #[test]
    fn test_call_unknown_method() {
        let result = call("UnknownMethod", "{}");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["error"].is_string());
    }

    #[test]
    fn test_call_invalid_json() {
        let result = call("Multiply", "invalid json");
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["error"].is_string());
    }

    #[test]
    fn test_call_missing_param() {
        let result = call("Multiply", r#"{"base": 5}"#);
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["error"].is_string());
    }
}
