#include "structures/HashTable.hpp"
#include <stdexcept>
#include <utility>

namespace adt {

std::uint64_t HashTable::hash(const std::string& value, std::uint64_t seed){
    std::uint64_t h = seed;
    for (unsigned char c : value){
        h ^= c;
        h *= 1099511628211ULL;
    }
    return h;
}

bool HashTable::prime(std::size_t value){
    if (value < 2) return false;
    for (std::size_t i = 2; i <= value / i; i++) if (value % i == 0) return false;
    return true;
}

std::size_t HashTable::capacityFor(std::size_t value){
    if (value < 5) value = 5;
    while (!prime(value)) value++;
    return value;
}

HashTable::HashTable(std::size_t capacity, bool doubleHash) : used(0), doubleHash(doubleHash), probes(0), collisions(0){
    std::size_t n = capacityFor(capacity);
    for (std::size_t i = 0; i < n; i++){
        keys.push("");
        values.push("");
        states.push("E");
    }
}

std::size_t HashTable::size() const { return used; }

std::size_t HashTable::capacity() const { return keys.size(); }

bool HashTable::isDouble() const { return doubleHash; }

std::size_t HashTable::probeCount() const { return probes; }

std::size_t HashTable::collisionCount() const { return collisions; }

std::size_t HashTable::position(const std::string& key, std::size_t attempt) const {
    std::size_t n = capacity();
    std::size_t first = hash(key, 14695981039346656037ULL) % n;
    std::size_t step = doubleHash ? 1 + hash(key, 7809847782465536322ULL) % (n - 1) : 1;
    return (first + attempt * step) % n;
}

void HashTable::grow(){
    HashTable bigger(capacity() * 2, doubleHash);
    for (const std::pair<std::string, std::string>& item : entries()) bigger.set(item.first, item.second);
    keys = std::move(bigger.keys);
    values = std::move(bigger.values);
    states = std::move(bigger.states);
    used = bigger.used;
}

bool HashTable::set(const std::string& key, const std::string& value){
    if ((used + 1) * 10 >= capacity() * 7) grow();
    std::size_t deleted = capacity();
    for (std::size_t i = 0; i < capacity(); i++){
        std::size_t p = position(key, i);
        probes++;
        std::string state = states.get(p);
        if (state == "O" && keys.get(p) == key){ values.set(p, value); return false; }
        if (state == "D" && deleted == capacity()) deleted = p;
        if (state == "E"){
            if (deleted != capacity()) p = deleted;
            keys.set(p, key);
            values.set(p, value);
            states.set(p, "O");
            used++;
            return true;
        }
        collisions++;
    }
    if (deleted != capacity()){
        keys.set(deleted, key);
        values.set(deleted, value);
        states.set(deleted, "O");
        used++;
        return true;
    }
    throw std::runtime_error("hash table is full");
}

bool HashTable::get(const std::string& key, std::string& value) const {
    for (std::size_t i = 0; i < capacity(); i++){
        std::size_t p = position(key, i);
        probes++;
        std::string state = states.get(p);
        if (state == "E") return false;
        if (state == "O" && keys.get(p) == key){ value = values.get(p); return true; }
        collisions++;
    }
    return false;
}

bool HashTable::erase(const std::string& key){
    for (std::size_t i = 0; i < capacity(); i++){
        std::size_t p = position(key, i);
        probes++;
        std::string state = states.get(p);
        if (state == "E") return false;
        if (state == "O" && keys.get(p) == key){ states.set(p, "D"); used--; return true; }
        collisions++;
    }
    return false;
}

std::vector<std::pair<std::string, std::string>> HashTable::entries() const {
    std::vector<std::pair<std::string, std::string>> result;
    for (std::size_t i = 0; i < capacity(); i++) if (states.get(i) == "O") result.push_back({keys.get(i), values.get(i)});
    return result;
}
}
