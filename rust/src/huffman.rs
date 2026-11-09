use crate::array::DynamicArray;
use crate::hash_table::HashTable;

struct HuffNode {
    symbol: u8,
    weight: usize,
    left: Option<usize>,
    right: Option<usize>,
}
fn smallest(active: &DynamicArray, nodes: &[HuffNode]) -> usize {
    let mut best = 0;
    for i in 1..active.len() {
        let current: usize = active.get(i).unwrap().parse().unwrap();
        let selected: usize = active.get(best).unwrap().parse().unwrap();
        if nodes[current].weight < nodes[selected].weight ||
            (nodes[current].weight == nodes[selected].weight && current < selected) { best = i; }
    }
    best
}
fn huffman_walk(nodes: &[HuffNode], id: usize, bits: String, codes: &mut [String]) {
    let node = &nodes[id];
    if let (Some(left), Some(right)) = (node.left, node.right) {
        huffman_walk(nodes, left, format!("{bits}0"), codes);
        huffman_walk(nodes, right, format!("{bits}1"), codes);
    } else { codes[node.symbol as usize] = if bits.is_empty() { "0".into() } else { bits }; }
}
pub fn huffman_encode(text: &str) -> Result<String, String> {
    if text.is_empty() { return Err("empty text".into()); }
    if !text.is_ascii() { return Err("expected ASCII text".into()); }
    let mut counts = [0usize; 256];
    for byte in text.bytes() { counts[byte as usize] += 1; }
    let mut nodes = Vec::new();
    let mut active = DynamicArray::default();
    for i in 0..256 {
        if counts[i] > 0 {
            nodes.push(HuffNode { symbol: i as u8, weight: counts[i], left: None, right: None });
            active.push((nodes.len() - 1).to_string());
        }
    }
    while active.len() > 1 {
        let index = smallest(&active, &nodes);
        let left: usize = active.erase(index)?.parse().unwrap();
        let index = smallest(&active, &nodes);
        let right: usize = active.erase(index)?.parse().unwrap();
        let weight = nodes[left].weight + nodes[right].weight;
        nodes.push(HuffNode { symbol: 0, weight, left: Some(left), right: Some(right) });
        active.push((nodes.len() - 1).to_string());
    }
    let mut codes = vec![String::new(); 256];
    let root: usize = active.get(0)?.parse().unwrap();
    huffman_walk(&nodes, root, String::new(), &mut codes);
    let mut result = String::from("CODE ");
    for byte in text.bytes() { result.push_str(&codes[byte as usize]); }
    result.push_str("\nTABLE");
    for i in 0..256 {
        if !codes[i].is_empty() { result.push_str(&format!(" {}:{}", i as u8 as char, codes[i])); }
    }
    Ok(result)
}
pub fn huffman_decode(args: &[&str]) -> Result<String, String> {
    if args.len() < 3 { return Err("missing command argument".into()); }
    let bits = args[1];
    if !bits.bytes().all(|b| b == b'0' || b == b'1') { return Err("invalid bit sequence".into()); }
    let mut codes = HashTable::new(17, false);
    let mut keys = Vec::new();
    for token in &args[2..] {
        let (symbol, code) = token.split_once(':').ok_or("invalid code table")?;
        let byte = if symbol.len() == 1 { symbol.as_bytes()[0] }
            else { symbol.parse::<u8>().map_err(|_| "invalid code table")? };
        if code.is_empty() || !code.bytes().all(|b| b == b'0' || b == b'1') || codes.get(code).is_some() {
            return Err("invalid code table".into());
        }
        codes.set(code, byte.to_string());
        keys.push(code.to_string());
    }
    for first in &keys {
        for second in &keys {
            if first.len() < second.len() && second.starts_with(first) { return Err("code table is not prefix-free".into()); }
        }
    }
    let mut result = Vec::new();
    let mut current = String::new();
    for bit in bits.chars() {
        current.push(bit);
        if let Some(byte) = codes.get(&current) {
            result.push(byte.parse::<u8>().map_err(|_| "invalid code table")?);
            current.clear();
        }
    }
    if !current.is_empty() { return Err("incomplete bit sequence".into()); }
    String::from_utf8(result).map_err(|_| "invalid text encoding".into())
}
