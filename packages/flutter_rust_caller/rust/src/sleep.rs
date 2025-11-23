/// 跨平台 sleep 模块
/// 针对不同平台提供统一的 sleep 接口
/// - WASM: 异步 sleep（返回 Future）
/// - 其他平台: 使用标准库的 thread::sleep

/// 同步 sleep（仅适用于非 WASM 平台）
#[cfg(not(target_arch = "wasm32"))]
pub fn sleep_millis(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

/// WASM 异步 sleep（仅在 WASM 中可用）
#[cfg(target_arch = "wasm32")]
pub async fn sleep_async(ms: u32) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let window = web_sys::window().expect("no global `window` exists");

        let closure = Closure::once(move || {
            let _ = resolve.call0(&JsValue::null());
        });

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                ms as i32,
            )
            .expect("failed to set timeout");

        closure.forget();
    });

    JsFuture::from(promise).await.ok();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_sleep_millis() {
        let start = Instant::now();
        sleep_millis(50);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 50);
    }
}
