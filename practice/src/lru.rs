
struct SimpleCache {
    queue: Vec<i32>,
    size: i32,
}
//single thread


// queue
pub fn init(sz: i32, sc: SimpleCache){
    sc.size = sz;
}

pub fn access(a:i32)->i32{
    let mut vv: Vec<i32> = Vec::new();
    vv.push(a);
}
pub fn evict()->i32{
    return 1;
}

// 1. 如何new一个结构体
// 2. 如何测试性能？
#[cfg(test)]
mod tests {
    use super::SimpleCache;
    #[test]
    pub fn test_lru_k() {
        let mut lru = SimpleCache::new(3);
        lru.put(10, 2000);
        lru.put(11, 2001);
        lru.put(12, 2002);
        assert_eq!(lru.get(10), 2000);
        lru.put(13, 2003);
        assert_eq!(lru.get(9), -1);
    }
}