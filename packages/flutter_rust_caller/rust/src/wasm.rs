// WASM Web 导出模块
// 为 JavaScript 环境导出 Rust 函数和状态

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

use crate::call::{call, call_async};

/// 同步调用 Rust 函数
/// 接受方法名和 JSON 参数字符串，返回 JSON 结果字符串
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn rust_call_wasm(method: &str, param_json: &str) -> String {
    call(method, param_json)
}

/// 异步调用 Rust 函数
/// 接受方法名和 JSON 参数字符串，返回 Promise
/// 通用的异步包装，将异步调用转换为 Promise，无任何特殊处理
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn rust_call_async_wasm(method: &str, param_json: &str) -> js_sys::Promise {
    let method_str = method.to_string();
    let param_str = param_json.to_string();

    js_sys::Promise::new(&mut |resolve, _reject| {
        let method = method_str.clone();
        let param = param_str.clone();
        
        spawn_local(async move {
            let result = call_async(&method, &param).await;
            let _ = resolve.call1(&JsValue::null(), &JsValue::from_str(&result));
        });
    })
}

/// 设置全局 WASM 准备标志
/// 从 JavaScript 调用以表示 WASM 已加载并可用
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_ready() {
    // 此函数由 JavaScript 调用，标记 WASM 已准备
}

