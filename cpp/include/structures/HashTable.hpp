#pragma once
#include "Array.hpp"
#include <cstddef>
#include <cstdint>
#include <string>
#include <utility>
#include <vector>

namespace adt {
class HashTable {
    Array keys;
    Array values;
    Array states;
    std::size_t used;
    bool doubleHash;
    mutable std::size_t probes;
    mutable std::size_t collisions;
    static std::uint64_t hash(const std::string& value, std::uint64_t seed);
    static bool prime(std::size_t value);
    static std::size_t capacityFor(std::size_t value);
    std::size_t position(const std::string& key, std::size_t attempt) const;
    void grow();
public:
    HashTable(std::size_t capacity = 17, bool doubleHash = false);
    std::size_t size() const;
    std::size_t capacity() const;
    bool isDouble() const;
    std::size_t probeCount() const;
    std::size_t collisionCount() const;
    bool set(const std::string& key, const std::string& value);
    bool get(const std::string& key, std::string& value) const;
    bool erase(const std::string& key);
    std::vector<std::pair<std::string, std::string>> entries() const;
};
}
