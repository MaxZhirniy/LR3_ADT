struct Node {
    value: String,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Default)]
pub struct DoublyList {
    nodes: Vec<Option<Node>>,
    free: Vec<usize>,
    head: Option<usize>,
    tail: Option<usize>,
    size: usize,
}

impl DoublyList {
    pub fn len(&self) -> usize { self.size }
    fn allocate(&mut self, node: Node) -> usize {
        if let Some(i) = self.free.pop() { self.nodes[i] = Some(node); i }
        else { self.nodes.push(Some(node)); self.nodes.len() - 1 }
    }
    fn node_at(&self, index: usize) -> Result<usize, String> {
        if index >= self.size { return Err("index out of range".into()); }
        if index <= self.size / 2 {
            let mut id = self.head.unwrap();
            for _ in 0..index { id = self.nodes[id].as_ref().unwrap().next.unwrap(); }
            Ok(id)
        } else {
            let mut id = self.tail.unwrap();
            for _ in 0..(self.size - index - 1) {
                id = self.nodes[id].as_ref().unwrap().prev.unwrap();
            }
            Ok(id)
        }
    }
    fn link(&mut self, prev: Option<usize>, next: Option<usize>, value: String) {
        let id = self.allocate(Node { value, prev, next });
        if let Some(p) = prev { self.nodes[p].as_mut().unwrap().next = Some(id); }
        else { self.head = Some(id); }
        if let Some(n) = next { self.nodes[n].as_mut().unwrap().prev = Some(id); }
        else { self.tail = Some(id); }
        self.size += 1;
    }
    pub fn push_back(&mut self, value: String) { let tail = self.tail; self.link(tail, None, value); }
    pub fn insert(&mut self, where_: &str, target: &str, value: String) -> Result<(), String> {
        if where_ == "HEAD" { let head = self.head; self.link(None, head, value); return Ok(()); }
        if where_ == "TAIL" { self.push_back(value); return Ok(()); }
        if where_ != "BEFORE" && where_ != "AFTER" { return Err("invalid position".into()); }
        let mut current = self.head;
        while let Some(id) = current {
            let node = self.nodes[id].as_ref().unwrap();
            if node.value == target {
                let prev = if where_ == "BEFORE" { node.prev } else { Some(id) };
                let next = if where_ == "BEFORE" { Some(id) } else { node.next };
                self.link(prev, next, value);
                return Ok(());
            }
            current = node.next;
        }
        Err("target not found".into())
    }
    fn unlink(&mut self, id: usize) -> String {
        let node = self.nodes[id].take().unwrap();
        if let Some(p) = node.prev { self.nodes[p].as_mut().unwrap().next = node.next; }
        else { self.head = node.next; }
        if let Some(n) = node.next { self.nodes[n].as_mut().unwrap().prev = node.prev; }
        else { self.tail = node.prev; }
        self.free.push(id);
        self.size -= 1;
        node.value
    }
    pub fn erase(&mut self, where_: &str, target: &str) -> Result<String, String> {
        if where_ != "HEAD" && where_ != "TAIL" && where_ != "BEFORE" && where_ != "AFTER" {
            return Err("invalid position".into());
        }
        if self.head.is_none() { return Err("list is empty".into()); }
        if where_ == "HEAD" { let id = self.head.unwrap(); return Ok(self.unlink(id)); }
        if where_ == "TAIL" { let id = self.tail.unwrap(); return Ok(self.unlink(id)); }
        let mut current = self.head;
        while let Some(id) = current {
            let node = self.nodes[id].as_ref().unwrap();
            if node.value == target {
                let remove_id = if where_ == "BEFORE" {
                    node.prev.ok_or("no element before target")?
                } else {
                    node.next.ok_or("no element after target")?
                };
                return Ok(self.unlink(remove_id));
            }
            current = node.next;
        }
        Err("target not found".into())
    }
    pub fn get(&self, index: usize) -> Result<String, String> {
        let id = self.node_at(index)?;
        Ok(self.nodes[id].as_ref().unwrap().value.clone())
    }
    pub fn find(&self, value: &str) -> Option<usize> {
        let mut current = self.head;
        let mut index = 0;
        while let Some(id) = current {
            let node = self.nodes[id].as_ref().unwrap();
            if node.value == value { return Some(index); }
            current = node.next;
            index += 1;
        }
        None
    }
    pub fn remove_value(&mut self, value: &str) -> bool {
        let mut current = self.head;
        while let Some(id) = current {
            let node = self.nodes[id].as_ref().unwrap();
            if node.value == value {
                self.unlink(id);
                return true;
            }
            current = node.next;
        }
        false
    }
    pub fn values(&self, reverse: bool) -> Vec<String> {
        let mut result = Vec::with_capacity(self.size);
        let mut current = if reverse { self.tail } else { self.head };
        while let Some(id) = current {
            let node = self.nodes[id].as_ref().unwrap();
            result.push(node.value.clone());
            current = if reverse { node.prev } else { node.next };
        }
        result
    }
}
