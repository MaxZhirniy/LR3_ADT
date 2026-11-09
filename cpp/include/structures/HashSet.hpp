#pragma once
#include "HashTable.hpp"
#include <string>
#include <vector>
namespace adt {
class HashSet {
    HashTable table;
public:
    HashSet();
    bool add(const std::string& value);
    bool remove(const std::string& value);
    bool contains(const std::string& value) const;
    std::vector<std::string> values() const;
};
}
