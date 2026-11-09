#include "Genealogy.hpp"
#include "structures/HashSet.hpp"
#include "structures/HashTable.hpp"
#include "structures/Array.hpp"
#include <algorithm>
#include <functional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using namespace std;
using adt::HashSet;
using adt::HashTable;
using adt::Array;

static string required(istringstream& in){
    string value;
    if (!(in >> value)) throw invalid_argument("missing command argument");
    return value;
}
static void finished(istringstream& in){
    string extra;
    if (in >> extra) throw invalid_argument("too many command arguments");
}
static size_t natural(const string& text){
    size_t used = 0;
    unsigned int value = stoull(text, &used);
    if (used != text.size() || text[0] == '-') throw invalid_argument("invalid number: " + text);
    return static_cast<size_t>(value);
}
string genealogy(istringstream& in){
    size_t n = natural(required(in));
    if (n == 0 || n > 100000) throw invalid_argument("invalid number of people");
    HashTable parents(n * 2 + 1, true);
    HashSet known;
    Array names;
    for (size_t i = 1; i < n; i++){
        string child = required(in), parent = required(in);
        if (child == parent) throw invalid_argument("person cannot be own parent");
        string existing;
        if (parents.get(child, existing)) throw invalid_argument("duplicate child");
        parents.set(child, parent);
        if (known.add(child)) names.push(child);
        if (known.add(parent)) names.push(parent);
    }
    finished(in);
    if (names.size() != n && n != 1) throw invalid_argument("incorrect family size");
    if (n == 1) throw invalid_argument("single person name is missing");
    vector<string> people = names.values();
    sort(people.begin(), people.end());
    vector<string> roots;
    for (const string& person : people){
        string parent;
        if (!parents.get(person, parent)) roots.push_back(person);
    }
    if (roots.size() != 1) throw invalid_argument("invalid family tree");
    HashTable counts(n * 2 + 1, false);
    for (const string& person : people) counts.set(person, "0");
    for (const string& person : people){
        string current = person, parent;
        HashSet visited;
        while (parents.get(current, parent)){
            if (!visited.add(current)) throw invalid_argument("cycle in family tree");
            string count;
            counts.get(parent, count);
            counts.set(parent, to_string(stoull(count) + 1));
            current = parent;
        }
    }
    string result;
    for (const string& person : people){
        string count;
        counts.get(person, count);
        result += person + " " + count + "\n";
    }
    result += "TREE\n";
    function<void(const string&, size_t)> walk = [&](const string& person, size_t depth){
        result += string(depth * 2, ' ') + person + "\n";
        for (const string& child : people){
            string parent;
            if (parents.get(child, parent) && parent == person) walk(child, depth + 1);
        }
    };
    walk(roots[0], 0);
    if (!result.empty() && result.back() == '\n') result.pop_back();
    return result;
}
