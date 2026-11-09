use crate::baseball::baseball;
use crate::genealogy::genealogy;
use crate::huffman::{huffman_encode, huffman_decode};
use crate::pattern_matcher::matches_pattern;
use crate::set_operations::combine_sets;
use crate::storage::{Container, Database};
use crate::hash_table::HashTable;
use crate::lru::LruCache;

fn required<'a>(args: &'a [&str], i: usize) -> Result<&'a str, String> {
    args.get(i).copied().ok_or_else(|| "missing command argument".into())
}
fn count(args: &[&str], n: usize) -> Result<(), String> {
    if args.len() == n { Ok(()) } else { Err("invalid number of command arguments".into()) }
}
fn natural(text: &str) -> Result<usize, String> {
    text.parse::<usize>().map_err(|_| format!("invalid number: {text}"))
}
fn bracket(values: &[String]) -> String { format!("[{}]", values.join(" ")) }

pub fn run(db: &mut Database, line: &str) -> Result<(String, bool), String> {
    let args: Vec<&str> = line.split_whitespace().collect();
    if args.is_empty() { return Ok((String::new(), false)); }
    let op = args[0].to_ascii_uppercase();
    if op == "HELP" {
        count(&args, 1)?;
        return Ok(("CREATE E name | DROP name | PRINT name | BASEBALL | SETADD/SETDEL/SET_AT/SETUNION/SETINTER/SETDIFF | MATCH | HUFFENC/HUFFDEC | HCREATE/HSET/HGET/HDEL/HREAD/HSTATS | GENEALOGY | LRUCREATE/LRUSET/LRUGET/LRUREAD | EXIT".into(), false));
    }
    if op == "CREATE" {
        count(&args, 3)?;
        if args[1].to_ascii_uppercase() != "E" { return Err("type must be E".into()); }
        let name = args[2];
        if db.contains_key(name) { return Err("name already exists".into()); }
        db.insert(name.into(), Container::new('E')?);
        return Ok(("OK".into(), true));
    }
    if op == "DROP" {
        count(&args, 2)?;
        let name = args[1];
        if db.remove(name).is_none() { return Err(format!("structure not found: {name}")); }
        return Ok(("OK".into(), true));
    }
    if op == "PRINT" {
        count(&args, 2)?;
        let c = db.get(args[1]).ok_or_else(|| format!("structure not found: {}", args[1]))?;
        return Ok((bracket(&c.values()), false));
    }
    if op == "BASEBALL" { return Ok((baseball(&args)?, false)); }
    if op == "MATCH" {
        count(&args, 3)?;
        return Ok((if matches_pattern(args[1], args[2]) { "TRUE" } else { "FALSE" }.into(), false));
    }
    if op == "HUFFENC" { count(&args, 2)?; return Ok((huffman_encode(args[1])?, false)); }
    if op == "HUFFDEC" { return Ok((huffman_decode(&args)?, false)); }
    if op == "GENEALOGY" { return Ok((genealogy(&args)?, false)); }
    if op == "SETADD" || op == "SETDEL" || op == "SET_AT" {
        count(&args, 3)?;
        let name = args[1];
        let value = args[2];
        let set = match db.get_mut(name) { Some(Container::Set(x)) => x, _ => return Err(format!("set not found: {name}")) };
        if op == "SET_AT" { return Ok((if set.contains(value) { "TRUE" } else { "FALSE" }.into(), false)); }
        let changed = if op == "SETADD" { set.add(value) } else { set.remove(value) };
        return Ok((if changed { "TRUE" } else { "FALSE" }.into(), changed));
    }
    if op == "SETUNION" || op == "SETINTER" || op == "SETDIFF" {
        count(&args, 4)?;
        let name = args[3];
        if db.contains_key(name) { return Err("name already exists".into()); }
        let left = match db.get(args[1]) { Some(Container::Set(x)) => x, _ => return Err("set not found".into()) };
        let right = match db.get(args[2]) { Some(Container::Set(x)) => x, _ => return Err("set not found".into()) };
        let result = combine_sets(left, right, &op);
        db.insert(name.into(), Container::Set(result));
        return Ok(("OK".into(), true));
    }
    if op == "HCREATE" {
        count(&args, 4)?;
        let mode = args[1].to_ascii_uppercase();
        let name = args[2];
        let capacity = natural(args[3])?;
        if (mode != "LINEAR" && mode != "DOUBLE") || capacity > 1_000_000 { return Err("invalid hash table parameters".into()); }
        if db.contains_key(name) { return Err("name already exists".into()); }
        db.insert(name.into(), Container::Hash(HashTable::new(capacity, mode == "DOUBLE")));
        return Ok(("OK".into(), true));
    }
    if op == "HSET" || op == "HGET" || op == "HDEL" || op == "HREAD" || op == "HSTATS" {
        let name = required(&args, 1)?;
        let table = match db.get_mut(name) { Some(Container::Hash(x)) => x, _ => return Err(format!("hash table not found: {name}")) };
        if op == "HREAD" {
            count(&args, 2)?;
            let mut pairs: Vec<String> = table.entries().into_iter().map(|(k,v)| format!("{k}:{v}")).collect();
            pairs.sort();
            return Ok((bracket(&pairs), false));
        }
        if op == "HSTATS" {
            count(&args, 2)?;
            return Ok((format!("SIZE {} CAPACITY {} PROBES {} COLLISIONS {}", table.len(), table.capacity(), table.probes(), table.collisions()), false));
        }
        if op == "HSET" { count(&args, 4)?; table.set(args[2], args[3].into()); return Ok(("OK".into(), true)); }
        if op == "HDEL" { count(&args, 3)?; let changed = table.erase(args[2]); return Ok((if changed { "TRUE" } else { "FALSE" }.into(), changed)); }
        count(&args, 3)?;
        return Ok((table.get(args[2]).unwrap_or_else(|| "-1".into()), false));
    }
    if op == "LRUCREATE" {
        count(&args, 3)?;
        let name = args[1];
        let capacity = natural(args[2])?;
        if capacity > 1_000_000 || db.contains_key(name) { return Err("invalid cache parameters".into()); }
        db.insert(name.into(), Container::Lru(LruCache::new(capacity)?));
        return Ok(("OK".into(), true));
    }
    if op == "LRUSET" || op == "LRUGET" || op == "LRUREAD" {
        let name = required(&args, 1)?;
        let cache = match db.get_mut(name) { Some(Container::Lru(x)) => x, _ => return Err(format!("cache not found: {name}")) };
        if op == "LRUREAD" {
            count(&args, 2)?;
            let pairs: Vec<String> = cache.entries().into_iter().map(|(k,v)| format!("{k}:{v}")).collect();
            return Ok((bracket(&pairs), false));
        }
        if op == "LRUSET" { count(&args, 4)?; cache.set(args[2], args[3].into()); return Ok(("OK".into(), true)); }
        count(&args, 3)?;
        let value = cache.get(args[2]);
        return Ok((value.clone().unwrap_or_else(|| "-1".into()), value.is_some()));
    }
    Err(format!("unknown command: {op}"))
}
