#pragma once
#include "structures/HashSet.hpp"
#include "structures/HashTable.hpp"
#include "structures/LruCache.hpp"
#include <map>
#include <string>
#include <variant>
#include <vector>

using adt::HashSet;
using adt::HashTable;
using adt::LruCache;

using Container = std::variant<HashSet, HashTable, LruCache>;
using Database = std::map<std::string, Container>;

char kindOf(const Container& data);
Container makeContainer(char type);
std::vector<std::string> getValues(const Container& data);
void loadData(const std::string& path, Database& db);
void saveData(const std::string& path, const Database& db);
