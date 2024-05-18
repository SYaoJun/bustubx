use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct LRUReplacer {
    capacity: u32,
    lru_map: Arc<Mutex<HashMap<u32, u32>>>,
}
/*
1. 结构体的创建，赋值，函数。
2. 用哈希表记录每个id的频次
*/
impl LRUReplacer {
    pub fn new(cap: u32) -> Self {
        LRUReplacer {
            capacity: cap,
            lru_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn victim(&self, frame_id: &u32) -> bool {
        return true;
    }

    pub fn pin(&self, frame_id: u32) {}

    // 什么时候调用 unpin呢？
    pub fn unpin(&self, frame_id: u32) {}

    pub fn size(&self) -> u32 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::LRUReplacer;
    #[test]
    pub fn test_lru_k() {
        let lru_replacer = LRUReplacer::new(7);

        lru_replacer.unpin(1);
        lru_replacer.unpin(2);
        lru_replacer.unpin(3);
        lru_replacer.unpin(4);
        lru_replacer.unpin(5);
        lru_replacer.unpin(6);
        lru_replacer.unpin(1);
        // 多次unpin有没有影响？
        assert_eq!(6, lru_replacer.size());
        // Scenario: get three victims from the lru.
        let value: u32 = 0;
        lru_replacer.victim(&value);
        assert_eq!(1, value);
        lru_replacer.victim(&value);
        assert_eq!(2, value);
        lru_replacer.victim(&value);
        assert_eq!(3, value);

        // Scenario: pin elements in the replacer.
        // Note that 3 has already been victimized, so pinning 3 should have no effect.
        lru_replacer.pin(3);
        lru_replacer.pin(4);
        assert_eq!(2, lru_replacer.size());

        // Scenario: unpin 4. We expect that the reference bit of 4 will be set to 1.
        lru_replacer.unpin(4);

        // Scenario: continue looking for victims. We expect these victims.
        lru_replacer.victim(&value);
        assert_eq!(5, value);
        lru_replacer.victim(&value);
        assert_eq!(6, value);
        lru_replacer.victim(&value);
        assert_eq!(4, value);
    }
}
