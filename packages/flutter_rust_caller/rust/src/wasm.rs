// WASM Web 导出模块
// 为 JavaScript 环境导出 Rust 函数和状态

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::call::call;

/// 同步调用 Rust 函数
/// 接受方法名和 JSON 参数字符串，返回 JSON 结果字符串
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn rust_call_wasm(method: &str, param_json: &str) -> String {
    call(method, param_json)
}

/// 异步调用 Rust 函数
/// 接受方法名和 JSON 参数字符串，返回 Promise (通过 JavaScript Promise 包装)
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn rust_call_async_wasm(method: &str, param_json: &str) -> JsValue {
    let result = call(method, param_json);
    // 返回一个已解决的 Promise，包含结果
    js_sys::Promise::resolve(&JsValue::from_str(&result)).into()
}

/// 设置全局 WASM 准备标志
/// 从 JavaScript 调用以表示 WASM 已加载并可用
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_ready() {
    // 此函数由 JavaScript 调用，标记 WASM 已准备
}

