use crate::array::DynamicArray;
use crate::hash_table::{HashSet, HashTable};

fn required<'a>(args: &'a [&str], i: usize) -> Result<&'a str, String> {
    args.get(i).copied().ok_or_else(|| "missing command argument".into())
}
fn natural(text: &str) -> Result<usize, String> {
    text.parse::<usize>().map_err(|_| format!("invalid number: {text}"))
}
fn tree(person: &str, depth: usize, people: &[String], parents: &HashTable, result: &mut String) {
    result.push_str(&format!("{}{}\n", " ".repeat(depth * 2), person));
    for child in people {
        if parents.get(child).as_deref() == Some(person) { tree(child, depth + 1, people, parents, result); }
    }
}
pub fn genealogy(args: &[&str]) -> Result<String, String> {
    let n = natural(required(args, 1)?)?;
    if n == 0 || n > 100000 || args.len() != 2 * n { return Err("invalid family size".into()); }
    let mut parents = HashTable::new(n * 2 + 1, true);
    let mut known = HashSet::new();
    let mut names = DynamicArray::default();
    for i in 0..n - 1 {
        let child = args[2 + i * 2];
        let parent = args[3 + i * 2];
        if child == parent || parents.get(child).is_some() { return Err("invalid parent relation".into()); }
        parents.set(child, parent.to_string());
        if known.add(child) { names.push(child.into()); }
        if known.add(parent) { names.push(parent.into()); }
    }
    if names.len() != n { return Err("incorrect family size".into()); }
    let mut people = names.values();
    people.sort();
    let roots: Vec<String> = people.iter().filter(|name| parents.get(name).is_none()).cloned().collect();
    if roots.len() != 1 { return Err("invalid family tree".into()); }
    let mut counts = HashTable::new(n * 2 + 1, false);
    for person in &people { counts.set(person, "0".into()); }
    for person in &people {
        let mut current = person.clone();
        let mut visited = HashSet::new();
        while let Some(parent) = parents.get(&current) {
            if !visited.add(&current) { return Err("cycle in family tree".into()); }
            let count: usize = counts.get(&parent).ok_or("unknown parent")?.parse().map_err(|_| "invalid count")?;
            counts.set(&parent, (count + 1).to_string());
            current = parent;
        }
    }
    let mut result = String::new();
    for person in &people { result.push_str(&format!("{} {}\n", person, counts.get(person).unwrap())); }
    result.push_str("TREE\n");
    tree(&roots[0], 0, &people, &parents, &mut result);
    if result.ends_with('\n') { result.pop(); }
    Ok(result)
}
