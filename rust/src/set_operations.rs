use crate::hash_table::HashSet;

pub fn combine_sets(left: &HashSet, right: &HashSet, operation: &str) -> HashSet {
    let mut result = HashSet::new();
    for value in left.values() {
        if operation == "SETUNION" || (operation == "SETINTER" && right.contains(&value)) || (operation == "SETDIFF" && !right.contains(&value)) { result.add(&value); }
    }
    if operation == "SETUNION" { for value in right.values() { result.add(&value); } }
    result
}
