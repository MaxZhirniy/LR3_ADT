use crate::array::DynamicArray;

pub fn matches_pattern(text: &str, pattern: &str) -> bool {
    let mut symbols = DynamicArray::default();
    for c in pattern.chars() { symbols.push(c.to_string()); }
    let source: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut j = 0;
    let mut last_star = symbols.len();
    let mut matched = 0;
    while i < source.len() {
        if j < symbols.len() && (symbols.get(j).unwrap() == "?" || symbols.get(j).unwrap() == source[i].to_string()) {
            i += 1;
            j += 1;
        } else if j < symbols.len() && symbols.get(j).unwrap() == "*" {
            last_star = j;
            j += 1;
            matched = i;
        } else if last_star < symbols.len() {
            j = last_star + 1;
            matched += 1;
            i = matched;
        } else { return false; }
    }
    while j < symbols.len() && symbols.get(j).unwrap() == "*" { j += 1; }
    j == symbols.len()
}
