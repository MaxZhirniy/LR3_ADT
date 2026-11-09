#include "structures/LruCache.hpp"
#include <stdexcept>

namespace adt {

LruCache::LruCache(std::size_t capacity) : data(17, false), limit(capacity){
    if (capacity == 0) throw std::invalid_argument("cache capacity must be positive");
}

std::size_t LruCache::capacity() const { return limit; }

std::size_t LruCache::size() const { return data.size(); }

void LruCache::set(const std::string& key, const std::string& value){
    std::string previous;
    if (data.get(key, previous)) order.remove(key);
    else if (data.size() == limit){
        std::string oldest = order.erase("HEAD", "");
        data.erase(oldest);
    }
    data.set(key, value);
    order.pushBack(key);
}

bool LruCache::get(const std::string& key, std::string& value){
    if (!data.get(key, value)) return false;
    order.remove(key);
    order.pushBack(key);
    return true;
}

std::vector<std::pair<std::string, std::string>> LruCache::entries() const {
    std::vector<std::pair<std::string, std::string>> result;
    for (const std::string& key : order.values()){
        std::string value;
        data.get(key, value);
        result.push_back({key, value});
    }
    return result;
}
}
