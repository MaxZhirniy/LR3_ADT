use crate::forward_list::ForwardList;

#[derive(Default)]
pub struct Stack { items: ForwardList }
impl Stack {
    pub fn push(&mut self, value: String) { self.items.insert("HEAD", "", value).unwrap(); }
    pub fn pop(&mut self) -> Result<String, String> { self.items.erase("HEAD", "") }
    pub fn top(&self) -> Result<String, String> { self.items.get(0) }
    pub fn values(&self) -> Vec<String> { self.items.values(false) }
}
