use crate::array::DynamicArray;
use std::cell::Cell;

pub struct HashTable {
    keys: DynamicArray,
    values: DynamicArray,
    states: DynamicArray,
    count: usize,
    double_hash: bool,
    probes: Cell<usize>,
    collisions: Cell<usize>,
}
impl HashTable {
    fn hash(text: &str, seed: u64) -> u64 {
        let mut value = seed;
        for byte in text.bytes() {
            value ^= byte as u64;
            value = value.wrapping_mul(1099511628211);
        }
        value
    }
    fn prime(n: usize) -> bool {
        if n < 2 { return false; }
        let mut i = 2;
        while i <= n / i {
            if n % i == 0 { return false; }
            i += 1;
        }
        true
    }
    pub fn new(capacity: usize, double_hash: bool) -> Self {
        let mut size = capacity.max(5);
        while !Self::prime(size) { size += 1; }
        let mut table = Self { keys: DynamicArray::default(), values: DynamicArray::default(), states: DynamicArray::default(), count: 0, double_hash, probes: Cell::new(0), collisions: Cell::new(0) };
        for _ in 0..size {
            table.keys.push(String::new());
            table.values.push(String::new());
            table.states.push("E".into());
        }
        table
    }
    pub fn len(&self) -> usize { self.count }
    pub fn capacity(&self) -> usize { self.keys.len() }
    pub fn is_double(&self) -> bool { self.double_hash }
    pub fn probes(&self) -> usize { self.probes.get() }
    pub fn collisions(&self) -> usize { self.collisions.get() }
    fn position(&self, key: &str, attempt: usize) -> usize {
        let n = self.capacity();
        let first = (Self::hash(key, 14695981039346656037) % n as u64) as usize;
        let step = if self.double_hash { 1 + (Self::hash(key, 7809847782465536322) % (n as u64 - 1)) as usize } else { 1 };
        (first + attempt.wrapping_mul(step)) % n
    }
    fn grow(&mut self) {
        let entries = self.entries();
        *self = Self::new(self.capacity() * 2, self.double_hash);
        for (key, value) in entries { self.set(&key, value); }
    }
    pub fn set(&mut self, key: &str, value: String) -> bool {
        if (self.count + 1) * 10 >= self.capacity() * 7 { self.grow(); }
        let mut deleted = None;
        for i in 0..self.capacity() {
            let mut p = self.position(key, i);
            self.probes.set(self.probes.get() + 1);
            let state = self.states.get(p).unwrap();
            if state == "O" && self.keys.get(p).unwrap() == key {
                self.values.set(p, value).unwrap();
                return false;
            }
            if state == "D" && deleted.is_none() { deleted = Some(p); }
            if state == "E" {
                if let Some(id) = deleted { p = id; }
                self.keys.set(p, key.into()).unwrap();
                self.values.set(p, value).unwrap();
                self.states.set(p, "O".into()).unwrap();
                self.count += 1;
                return true;
            }
            self.collisions.set(self.collisions.get() + 1);
        }
        if let Some(p) = deleted {
            self.keys.set(p, key.into()).unwrap();
            self.values.set(p, value).unwrap();
            self.states.set(p, "O".into()).unwrap();
            self.count += 1;
            return true;
        }
        panic!("hash table is full")
    }
    pub fn get(&self, key: &str) -> Option<String> {
        for i in 0..self.capacity() {
            let p = self.position(key, i);
            self.probes.set(self.probes.get() + 1);
            let state = self.states.get(p).unwrap();
            if state == "E" { return None; }
            if state == "O" && self.keys.get(p).unwrap() == key { return Some(self.values.get(p).unwrap()); }
            self.collisions.set(self.collisions.get() + 1);
        }
        None
    }
    pub fn erase(&mut self, key: &str) -> bool {
        for i in 0..self.capacity() {
            let p = self.position(key, i);
            self.probes.set(self.probes.get() + 1);
            let state = self.states.get(p).unwrap();
            if state == "E" { return false; }
            if state == "O" && self.keys.get(p).unwrap() == key {
                self.states.set(p, "D".into()).unwrap();
                self.count -= 1;
                return true;
            }
            self.collisions.set(self.collisions.get() + 1);
        }
        false
    }
    pub fn entries(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();
        for i in 0..self.capacity() {
            if self.states.get(i).unwrap() == "O" { result.push((self.keys.get(i).unwrap(), self.values.get(i).unwrap())); }
        }
        result
    }
}
pub struct HashSet { data: HashTable }
impl HashSet {
    pub fn new() -> Self { Self { data: HashTable::new(17, false) } }
    pub fn add(&mut self, value: &str) -> bool {
        if self.contains(value) { return false; }
        self.data.set(value, "1".into())
    }
    pub fn remove(&mut self, value: &str) -> bool { self.data.erase(value) }
    pub fn contains(&self, value: &str) -> bool { self.data.get(value).is_some() }
    pub fn values(&self) -> Vec<String> {
        let mut result: Vec<String> = self.data.entries().into_iter().map(|(key, _)| key).collect();
        result.sort();
        result
    }
}
