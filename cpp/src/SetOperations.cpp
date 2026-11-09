#include "SetOperations.hpp"
#include <string>
#include <utility>
using namespace std;
using adt::HashSet;

HashSet combineSets(const HashSet& left, const HashSet& right, const string& operation){
    HashSet result;
    for (const string& value : left.values()){
        if (operation == "SETUNION" || (operation == "SETINTER" && right.contains(value)) || (operation == "SETDIFF" && !right.contains(value))) result.add(value);
    }
    if (operation == "SETUNION") for (const string& value : right.values()) result.add(value);
    return result;
}
