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
pub fn sum_long_running(a: i32, b: i32) -> i32 {
    std::thread::sleep(std::time::Duration::from_millis(200));
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
