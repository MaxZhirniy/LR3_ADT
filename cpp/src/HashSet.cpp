#include "structures/HashSet.hpp"
#include <algorithm>

namespace adt {
HashSet::HashSet() : table(17, false) {}

bool HashSet::add(const std::string& value){
    if (contains(value)) return false;
    table.set(value, "1");
    return true;
}

bool HashSet::remove(const std::string& value){ 
    return table.erase(value); 
}

bool HashSet::contains(const std::string& value) const {
    std::string result;
    return table.get(value, result);
}

std::vector<std::string> HashSet::values() const {
    std::vector<std::string> result;
    for (const std::pair<std::string, std::string>& entry : table.entries()) result.push_back(entry.first);
    std::sort(result.begin(), result.end());
    return result;
}
}
