use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
}

/// Increase 每次调用将全局计数器加一并返回
pub fn increase() -> i32 {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
    *counter
}

/// Sum 返回两个数之和
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// SumLongRunning 模拟耗时操作，返回两个数之和
/// 非 WASM: 使用 thread::sleep
/// WASM: 直接返回（WASM 中不支持同步 sleep）
#[cfg(not(target_arch = "wasm32"))]
pub fn sum_long_running(a: i32, b: i32) -> i32 {
    use crate::sleep::sleep_millis;
    sleep_millis(200);
    a + b
}

#[cfg(target_arch = "wasm32")]
pub fn sum_long_running(a: i32, b: i32) -> i32 {
    a + b
}

/// SumLongRunning 异步版本（仅在 WASM 中可用）
/// 在异步上下文中执行耗时操作
#[cfg(target_arch = "wasm32")]
pub async fn sum_long_running_async(a: i32, b: i32) -> i32 {
    use crate::sleep::sleep_async;
    sleep_async(200).await;
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increase() {
        // Reset counter for testing
        assert!(increase() > 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
        assert_eq!(sum(-1, 1), 0);
        assert_eq!(sum(0, 0), 0);
    }

    #[test]
    fn test_sum_long_running() {
        assert_eq!(sum_long_running(10, 20), 30);
    }
}
