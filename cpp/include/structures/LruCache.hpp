#pragma once
#include "DoublyList.hpp"
#include "HashTable.hpp"
#include <cstddef>
#include <string>
#include <utility>
#include <vector>
namespace adt {
class LruCache {
    DoublyList order;
    HashTable data;
    std::size_t limit;
public:
    explicit LruCache(std::size_t capacity = 2);
    std::size_t capacity() const;
    std::size_t size() const;
    void set(const std::string& key, const std::string& value);
    bool get(const std::string& key, std::string& value);
    std::vector<std::pair<std::string, std::string>> entries() const;
};
}
