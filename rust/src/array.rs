#[derive(Default)]
pub struct DynamicArray {
    items: Vec<String>,
}

impl DynamicArray {
    pub fn len(&self) -> usize { self.items.len() }
    pub fn values(&self) -> Vec<String> { self.items.clone() }
    pub fn push(&mut self, value: String) { self.items.push(value); }
    pub fn insert(&mut self, index: usize, value: String) -> Result<(), String> {
        if index > self.len() { return Err("index out of range".into()); }
        self.items.insert(index, value);
        Ok(())
    }
    pub fn get(&self, index: usize) -> Result<String, String> {
        self.items.get(index).cloned().ok_or_else(|| "index out of range".into())
    }
    pub fn set(&mut self, index: usize, value: String) -> Result<(), String> {
        let slot = self.items.get_mut(index).ok_or("index out of range")?;
        *slot = value;
        Ok(())
    }
    pub fn erase(&mut self, index: usize) -> Result<String, String> {
        if index >= self.len() { return Err("index out of range".into()); }
        Ok(self.items.remove(index))
    }
}
