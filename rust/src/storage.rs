use crate::hash_table::{HashSet, HashTable};
use crate::lru::LruCache;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub enum Container {
    Set(HashSet),
    Hash(HashTable),
    Lru(LruCache),
}
impl Container {
    pub fn new(kind: char) -> Result<Self, String> {
        match kind {
            'E' => Ok(Self::Set(HashSet::new())),
            'H' => Ok(Self::Hash(HashTable::new(17, false))),
            'U' => Ok(Self::Lru(LruCache::new(2)?)),
            _ => Err("unknown structure type".into()),
        }
    }
    pub fn kind(&self) -> char {
        match self { Self::Set(_) => 'E', Self::Hash(_) => 'H', Self::Lru(_) => 'U' }
    }
    pub fn values(&self) -> Vec<String> {
        match self {
            Self::Set(x) => x.values(),
            Self::Hash(x) => x.entries().into_iter().map(|(k,v)| format!("{k}:{v}")).collect(),
            Self::Lru(x) => x.entries().into_iter().map(|(k,v)| format!("{k}:{v}")).collect(),
        }
    }
}
pub type Database = BTreeMap<String, Container>;

pub fn load(path: &Path) -> Result<Database, String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Database::new()),
        Err(e) => return Err(format!("cannot open {}: {e}", path.display())),
    };
    let mut db = Database::new();
    for (line_number, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|e| e.to_string())?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }
        if parts.len() < 3 { return Err(format!("invalid data at line {}", line_number + 1)); }
        let kind = parts[0].chars().next().ok_or("missing kind")?;
        if parts[0].len() != 1 { return Err("invalid structure kind".into()); }
        let name = parts[1];
        if db.contains_key(name) { return Err("duplicate structure name".into()); }
        if kind == 'H' {
            if parts.len() < 5 { return Err("invalid hash table".into()); }
            let mode = parts[2];
            if mode != "LINEAR" && mode != "DOUBLE" { return Err("invalid hash table mode".into()); }
            let capacity: usize = parts[3].parse().map_err(|_| "invalid capacity")?;
            let count: usize = parts[4].parse().map_err(|_| "invalid count")?;
            if capacity > 1_000_000 || count > 1_000_000 || parts.len() != 5 + 2 * count { return Err("incorrect hash table size".into()); }
            let mut table = HashTable::new(capacity, mode == "DOUBLE");
            for i in 0..count { table.set(parts[5 + 2 * i], parts[6 + 2 * i].into()); }
            db.insert(name.into(), Container::Hash(table));
            continue;
        }
        if kind == 'U' {
            if parts.len() < 4 { return Err("invalid cache".into()); }
            let capacity: usize = parts[2].parse().map_err(|_| "invalid capacity")?;
            let count: usize = parts[3].parse().map_err(|_| "invalid count")?;
            if capacity == 0 || capacity > 1_000_000 || count > capacity || parts.len() != 4 + 2 * count { return Err("incorrect cache size".into()); }
            let mut cache = LruCache::new(capacity)?;
            for i in 0..count { cache.set(parts[4 + 2 * i], parts[5 + 2 * i].into()); }
            db.insert(name.into(), Container::Lru(cache));
            continue;
        }
        if kind != 'E' { return Err("unknown structure type".into()); }
        let count: usize = parts[2].parse().map_err(|_| "invalid count")?;
        if count > 1_000_000 || parts.len() != count + 3 { return Err("incorrect element count".into()); }
        let mut c = Container::new(kind)?;
        if let Container::Set(x) = &mut c {
            for value in &parts[3..] { x.add(value); }
        }
        db.insert(name.to_string(), c);
    }
    Ok(db)
}

pub fn save(path: &Path, db: &Database) -> Result<(), String> {
    let temp = path.with_extension(format!("{}tmp", path.extension()
        .and_then(|s| s.to_str()).map(|s| format!("{s}.")).unwrap_or_default()));
    let mut file = File::create(&temp).map_err(|e| e.to_string())?;
    for (name, c) in db {
        if let Container::Hash(table) = c {
            let entries = table.entries();
            let mode = if table.is_double() { "DOUBLE" } else { "LINEAR" };
            write!(file, "H {name} {mode} {} {}", table.capacity(), entries.len()).map_err(|e| e.to_string())?;
            for (key, value) in entries { write!(file, " {key} {value}").map_err(|e| e.to_string())?; }
            writeln!(file).map_err(|e| e.to_string())?;
            continue;
        }
        if let Container::Lru(cache) = c {
            let entries = cache.entries();
            write!(file, "U {name} {} {}", cache.capacity(), entries.len()).map_err(|e| e.to_string())?;
            for (key, value) in entries { write!(file, " {key} {value}").map_err(|e| e.to_string())?; }
            writeln!(file).map_err(|e| e.to_string())?;
            continue;
        }
        let vals = c.values();
        write!(file, "{} {} {}", c.kind(), name, vals.len()).map_err(|e| e.to_string())?;
        for value in vals { write!(file, " {value}").map_err(|e| e.to_string())?; }
        writeln!(file).map_err(|e| e.to_string())?;
    }
    file.flush().map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    fs::rename(&temp, path).map_err(|e| format!("cannot replace data file: {e}"))
}
