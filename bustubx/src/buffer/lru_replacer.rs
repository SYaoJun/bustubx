use std::collections::{HashMap, LinkedList};

// 定义一个简单的函数，用于加法操作
fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[derive(Debug)]
pub struct LRU {
    frame_id: u32,
    // 链表 和 哈希表
    lru_list: LinkedList<u64>,
    lru_map: HashMap<i32, i32>,
}

impl LRU {
    pub fn new() {
        panic!();
    }

    pub fn Victim(frame_id: u32) {
        panic!();
    }

    pub fn Pin(frame_id: u32) {
        panic!();
    }

    pub fn Unpin(frame_id: u32) {
        panic!();
    }
    pub fn Size() {
        panic!();
    }
}
// 使用 #[test] 属性标记测试函数
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_addition() {
        // 在测试函数中调用被测试的函数，并使用断言宏进行断言
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
