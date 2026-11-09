#include "Commands.hpp"
#include "Baseball.hpp"
#include "PatternMatcher.hpp"
#include "Huffman.hpp"
#include "Genealogy.hpp"
#include "SetOperations.hpp"
#include <algorithm>
#include <cctype>
#include <sstream>
#include <stdexcept>
#include <utility>
using namespace std;

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
static string join(const vector<string>& items){
    string result = "[";
    for (size_t i = 0; i < items.size(); i++){
        if (i) result += ' ';
        result += items[i];
    }
    return result + "]";
}
static HashSet& setOf(Database& db, const string& name){
    Database::iterator it = db.find(name);
    if (it == db.end() || kindOf(it->second) != 'E') throw invalid_argument("set not found: " + name);
    return get<HashSet>(it->second);
}
static HashTable& tableOf(Database& db, const string& name){
    Database::iterator it = db.find(name);
    if (it == db.end() || kindOf(it->second) != 'H') throw invalid_argument("hash table not found: " + name);
    return get<HashTable>(it->second);
}
static LruCache& cacheOf(Database& db, const string& name){
    Database::iterator it = db.find(name);
    if (it == db.end() || kindOf(it->second) != 'U') throw invalid_argument("cache not found: " + name);
    return get<LruCache>(it->second);
}
string upperText(string s) { for (char& c : s) c = static_cast<char>(toupper(static_cast<unsigned char>(c))); return s; }

string runCommand(Database& db, const string& command, bool& changed){
    istringstream in(command);
    string op;
    if (!(in >> op)) return "";
    op = upperText(op);
    if (op == "HELP"){
        finished(in);
        return "CREATE E name | DROP name | PRINT name | BASEBALL | SETADD/SETDEL/SET_AT/SETUNION/SETINTER/SETDIFF | MATCH | HUFFENC/HUFFDEC | HCREATE/HSET/HGET/HDEL/HREAD/HSTATS | GENEALOGY | LRUCREATE/LRUSET/LRUGET/LRUREAD | EXIT";
    }
    if (op == "CREATE"){
        string type = upperText(required(in)), name = required(in);
        finished(in);
        if (type != "E") throw invalid_argument("type must be E");
        if (db.count(name)) throw invalid_argument("name already exists");
        db.emplace(name, makeContainer('E'));
        changed = true;
        return "OK";
    }
    if (op == "DROP"){
        string name = required(in);
        finished(in);
        if (db.erase(name) == 0) throw invalid_argument("structure not found: " + name);
        changed = true;
        return "OK";
    }
    if (op == "PRINT"){
        string name = required(in);
        finished(in);
        Database::iterator it = db.find(name);
        if (it == db.end()) throw invalid_argument("structure not found: " + name);
        return join(getValues(it->second));
    }
    if (op == "BASEBALL") return to_string(baseball(in));
    if (op == "MATCH"){
        string value = required(in), pattern = required(in);
        finished(in);
        return patternMatch(value, pattern) ? "TRUE" : "FALSE";
    }
    if (op == "HUFFENC"){
        string value = required(in);
        finished(in);
        return huffmanEncode(value);
    }
    if (op == "HUFFDEC") return huffmanDecode(in);
    if (op == "GENEALOGY") return genealogy(in);
    if (op == "SETADD" || op == "SETDEL" || op == "SET_AT"){
        string name = required(in), value = required(in);
        finished(in);
        HashSet& set = setOf(db, name);
        if (op == "SET_AT") return set.contains(value) ? "TRUE" : "FALSE";
        bool ok = op == "SETADD" ? set.add(value) : set.remove(value);
        changed = ok;
        return ok ? "TRUE" : "FALSE";
    }
    if (op == "SETUNION" || op == "SETINTER" || op == "SETDIFF"){
        string a = required(in), b = required(in), name = required(in);
        finished(in);
        if (db.count(name)) throw invalid_argument("name already exists");
        HashSet& left = setOf(db, a);
        HashSet& right = setOf(db, b);
        db.emplace(name, combineSets(left, right, op));
        changed = true;
        return "OK";
    }
    if (op == "HCREATE"){
        string mode = upperText(required(in)), name = required(in);
        size_t capacity = natural(required(in));
        finished(in);
        if (mode != "LINEAR" && mode != "DOUBLE") throw invalid_argument("expected LINEAR or DOUBLE");
        if (capacity > 1000000) throw invalid_argument("invalid table capacity");
        if (db.count(name)) throw invalid_argument("name already exists");
        db.emplace(name, HashTable(capacity, mode == "DOUBLE"));
        changed = true;
        return "OK";
    }
    if (op == "HSET" || op == "HGET" || op == "HDEL" || op == "HREAD" || op == "HSTATS"){
        string name = required(in);
        HashTable& table = tableOf(db, name);
        if (op == "HREAD"){
            finished(in);
            vector<string> pairs;
            for (const pair<string, string>& item : table.entries()) pairs.push_back(item.first + ':' + item.second);
            sort(pairs.begin(), pairs.end());
            return join(pairs);
        }
        if (op == "HSTATS"){
            finished(in);
            return "SIZE " + to_string(table.size()) + " CAPACITY " + to_string(table.capacity()) + " PROBES " + to_string(table.probeCount()) + " COLLISIONS " + to_string(table.collisionCount());
        }
        string key = required(in);
        if (op == "HSET"){
            string value = required(in);
            finished(in);
            table.set(key, value);
            changed = true;
            return "OK";
        }
        if (op == "HDEL"){
            finished(in);
            changed = table.erase(key);
            return changed ? "TRUE" : "FALSE";
        }
        finished(in);
        string value;
        return table.get(key, value) ? value : "-1";
    }
    if (op == "LRUCREATE"){
        string name = required(in);
        size_t capacity = natural(required(in));
        finished(in);
        if (capacity > 1000000) throw invalid_argument("invalid cache capacity");
        if (db.count(name)) throw invalid_argument("name already exists");
        db.emplace(name, LruCache(capacity));
        changed = true;
        return "OK";
    }
    if (op == "LRUSET" || op == "LRUGET" || op == "LRUREAD"){
        string name = required(in);
        LruCache& cache = cacheOf(db, name);
        if (op == "LRUREAD"){
            finished(in);
            vector<string> pairs;
            for (const pair<string, string>& item : cache.entries()) pairs.push_back(item.first + ':' + item.second);
            return join(pairs);
        }
        string key = required(in);
        if (op == "LRUSET"){
            string value = required(in);
            finished(in);
            cache.set(key, value);
            changed = true;
            return "OK";
        }
        finished(in);
        string value;
        bool ok = cache.get(key, value);
        changed = ok;
        return ok ? value : "-1";
    }
    throw invalid_argument("unknown command: " + op);
}
