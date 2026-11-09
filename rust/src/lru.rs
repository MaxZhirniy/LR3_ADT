use crate::doubly_list::DoublyList;
use crate::hash_table::HashTable;

pub struct LruCache {
    order: DoublyList,
    values: HashTable,
    capacity: usize,
}
impl LruCache {
    pub fn new(capacity: usize) -> Result<Self, String> {
        if capacity == 0 { return Err("cache capacity must be positive".into()); }
        Ok(Self { order: DoublyList::default(), values: HashTable::new(17, false), capacity })
    }
    pub fn capacity(&self) -> usize { self.capacity }
    pub fn set(&mut self, key: &str, value: String) {
        if self.values.get(key).is_some() { self.order.remove_value(key); }
        else if self.values.len() == self.capacity {
            let old = self.order.erase("HEAD", "").unwrap();
            self.values.erase(&old);
        }
        self.values.set(key, value);
        self.order.push_back(key.into());
    }
    pub fn get(&mut self, key: &str) -> Option<String> {
        let value = self.values.get(key)?;
        self.order.remove_value(key);
        self.order.push_back(key.into());
        Some(value)
    }
    pub fn entries(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for key in self.order.values(false) {
            result.push((key.clone(), self.values.get(&key).unwrap()));
        }
        result
    }
}
