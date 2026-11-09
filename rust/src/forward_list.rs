struct Node {
    value: String,
    next: Option<Box<Node>>,
}

#[derive(Default)]
pub struct ForwardList {
    head: Option<Box<Node>>,
    size: usize,
}

impl ForwardList {
    pub fn len(&self) -> usize { self.size }

    pub fn push_back(&mut self, value: String) {
        let mut link = &mut self.head;
        loop {
            match link {
                Some(node) => link = &mut node.next,
                None => { *link = Some(Box::new(Node { value, next: None })); break; }
            }
        }
        self.size += 1;
    }
    fn unlink(link: &mut Option<Box<Node>>) -> String {
        let mut removed = link.take().unwrap();
        *link = removed.next.take();
        removed.value
    }
    pub fn insert(&mut self, where_: &str, target: &str, value: String) -> Result<(), String> {
        if where_ == "HEAD" {
            let next = self.head.take();
            self.head = Some(Box::new(Node { value, next }));
            self.size += 1;
            return Ok(());
        }
        if where_ == "TAIL" { self.push_back(value); return Ok(()); }
        if where_ != "BEFORE" && where_ != "AFTER" { return Err("invalid position".into()); }
        if self.head.is_none() { return Err("target not found".into()); }
        if where_ == "BEFORE" {
            let mut link = &mut self.head;
            loop {
                if link.as_ref().is_some_and(|node| node.value == target) {
                    let next = link.take();
                    *link = Some(Box::new(Node { value, next }));
                    self.size += 1;
                    return Ok(());
                }
                match link {
                    Some(node) => link = &mut node.next,
                    None => return Err("target not found".into()),
                }
            }
        }
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if node.value == target {
                let next = node.next.take();
                node.next = Some(Box::new(Node { value, next }));
                self.size += 1;
                return Ok(());
            }
            current = node.next.as_mut();
        }
        Err("target not found".into())
    }
    pub fn erase(&mut self, where_: &str, target: &str) -> Result<String, String> {
        if where_ != "HEAD" && where_ != "TAIL" && where_ != "BEFORE" && where_ != "AFTER" {
            return Err("invalid position".into());
        }
        if self.head.is_none() { return Err("list is empty".into()); }
        if where_ == "HEAD" {
            let value = Self::unlink(&mut self.head);
            self.size -= 1;
            return Ok(value);
        }
        if where_ == "TAIL" {
            let mut link = &mut self.head;
            loop {
                if link.as_ref().unwrap().next.is_none() {
                    let value = Self::unlink(link);
                    self.size -= 1;
                    return Ok(value);
                }
                link = &mut link.as_mut().unwrap().next;
            }
        }
        if where_ == "BEFORE" && self.head.as_ref().unwrap().value == target {
            return Err("no element before target".into());
        }
        let mut link = &mut self.head;
        loop {
            if where_ == "BEFORE" {
                if link.as_ref().is_some_and(|node| node.next.as_ref().is_some_and(|next| next.value == target)) {
                    let value = Self::unlink(link);
                    self.size -= 1;
                    return Ok(value);
                }
            } else if link.as_ref().is_some_and(|node| node.value == target) {
                if link.as_ref().unwrap().next.is_none() { return Err("no element after target".into()); }
                let value = Self::unlink(&mut link.as_mut().unwrap().next);
                self.size -= 1;
                return Ok(value);
            }
            match link {
                Some(node) => link = &mut node.next,
                None => return Err("target not found".into()),
            }
        }
    }
    pub fn get(&self, index: usize) -> Result<String, String> {
        if index >= self.size { return Err("index out of range".into()); }
        let mut node = self.head.as_deref();
        for _ in 0..index { node = node.unwrap().next.as_deref(); }
        Ok(node.unwrap().value.clone())
    }
    pub fn find(&self, value: &str) -> Option<usize> {
        let mut node = self.head.as_deref();
        let mut index = 0;
        while let Some(item) = node {
            if item.value == value { return Some(index); }
            index += 1;
            node = item.next.as_deref();
        }
        None
    }
    pub fn remove_value(&mut self, value: &str) -> bool {
        let mut link = &mut self.head;
        loop {
            if link.as_ref().is_some_and(|node| node.value == value) {
                Self::unlink(link);
                self.size -= 1;
                return true;
            }
            match link {
                Some(node) => link = &mut node.next,
                None => return false,
            }
        }
    }
    pub fn values(&self, reverse: bool) -> Vec<String> {
        let mut result = Vec::with_capacity(self.size);
        let mut current = self.head.as_deref();
        while let Some(item) = current {
            result.push(item.value.clone());
            current = item.next.as_deref();
        }
        if reverse { result.reverse(); }
        result
    }
}
