use std::sync::Mutex;

lazy_static::lazy_static! {
    // 静态保存最后一次质数计算的结果
    static ref LAST_PRIME: Mutex<i64> = Mutex::new(0);
}

/// Multiply 将数字乘以另一个数字（两参数）
pub fn multiply(base: i64, multiplier: i64) -> i64 {
    base * multiplier
}

/// PowerMultiply 计算 base * (exponent_base ^ exponent)（三参数）
pub fn power_multiply(base: i64, exponent_base: i64, exponent: i64) -> i64 {
    let power_result = exponent_base.pow(exponent as u32);
    base * power_result
}

/// 判断一个数是否为质数（最简单粗暴的循环判断）
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let mut i = 3i64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// NextPrime 从给定数字开始循环查找质数，直到找到指定数量的质数
/// base: 起始数字
/// count: 要找的质数个数（例如 count=100 表示找 100 个质数）
/// 返回最后一个找到的质数，并将其保存到静态变量
pub fn next_prime(base: i64, count: i64) -> i64 {
    let mut candidate = base + 1;
    let mut found_count = 0i64;
    let mut last_found = base;
    
    while found_count < count {
        if is_prime(candidate) {
            found_count += 1;
            last_found = candidate;
        }
        candidate += 1;
    }
    
    // 将最后一个找到的质数保存到全局静态变量
    let mut last_prime = LAST_PRIME.lock().unwrap();
    *last_prime = last_found;
    last_found
}

/// GetLastPrime 获取上次计算保存的质数结果
pub fn get_last_prime() -> i64 {
    *LAST_PRIME.lock().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(5, 10), 50);
        assert_eq!(multiply(3, 0), 0);
    }

    #[test]
    fn test_power_multiply() {
        // 10 * (10 ^ 10) = 10 * 10000000000 = 100000000000
        assert_eq!(power_multiply(10, 10, 10), 100000000000);
        assert_eq!(power_multiply(5, 2, 3), 40);  // 5 * (2^3) = 5 * 8 = 40
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(10, 1), 11);  // 从10开始找1个质数，结果是11
        assert_eq!(next_prime(20, 3), 31);  // 从20开始找3个质数：23, 29, 31，结果是31
    }
}
