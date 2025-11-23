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
/// 
/// 非 WASM 平台（macOS, iOS, Android, Linux）：
///   使用 thread::sleep 实现同步延迟，无论同步还是异步调用都会阻塞等待
/// 
/// WASM 平台的限制：
///   WASM 运行在单线程的 JavaScript 环境中，不支持同步阻塞（thread::sleep）
///   同步函数无法调用异步 sleep（.await），因为 Rust 语法限制
///   因此在同步调用中无法实现延迟，只能直接返回
///   
///   异步调用的延迟通过 sum_long_running_async 实现
#[cfg(not(target_arch = "wasm32"))]
pub fn sum_long_running(a: i32, b: i32) -> i32 {
    use crate::sleep::sleep_millis;
    sleep_millis(200);
    a + b
}

#[cfg(target_arch = "wasm32")]
pub fn sum_long_running(a: i32, b: i32) -> i32 {
    // WASM 同步调用无法实现 sleep，见上方注释
    a + b
}

/// SumLongRunning 异步版本（仅在 WASM 中可用）
/// 在异步上下文中可以使用 .await 调用异步 sleep
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
