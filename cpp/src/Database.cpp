#include "Database.hpp"
#include <algorithm>
#include <cstdio>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <utility>
using namespace std;

char kindOf(const Container& c) {
    switch (c.index()) { case 0: return 'E'; case 1: return 'H'; default: return 'U'; }
}
Container makeContainer(char k) {
    switch (k) { case 'E': return HashSet{}; case 'H': return HashTable{}; case 'U': return LruCache{}; default: throw invalid_argument("unknown structure type"); }
}
vector<string> getValues(const Container& c) {
    switch (kindOf(c)) {
        case 'E': return get<HashSet>(c).values();
        case 'H': {
            vector<string> result;
            for (const pair<string, string>& item : get<HashTable>(c).entries()) result.push_back(item.first + ':' + item.second);
            return result;
        }
        case 'U': {
            vector<string> result;
            for (const pair<string, string>& item : get<LruCache>(c).entries()) result.push_back(item.first + ':' + item.second);
            return result;
        }
        default: throw invalid_argument("unknown structure type");
    }
}
void saveData(const string& path, const Database& db) {
    const string tmp = path + ".tmp";
    ofstream out(tmp, ios::trunc);
    if (!out) throw runtime_error("cannot write: " + tmp);
    for (Database::const_iterator it = db.begin(); it != db.end(); ++it) {
        const string& name = it->first;
        const Container& data = it->second;
        if (kindOf(data) == 'H'){
            const HashTable& table = get<HashTable>(data);
            vector<pair<string, string>> entries = table.entries();
            out << 'H' << ' ' << name << ' ' << (table.isDouble() ? "DOUBLE" : "LINEAR") << ' ' << table.capacity() << ' ' << entries.size();
            for (const pair<string, string>& entry : entries) out << ' ' << entry.first << ' ' << entry.second;
            out << '\n';
            continue;
        }
        if (kindOf(data) == 'U'){
            const LruCache& cache = get<LruCache>(data);
            vector<pair<string, string>> entries = cache.entries();
            out << 'U' << ' ' << name << ' ' << cache.capacity() << ' ' << entries.size();
            for (const pair<string, string>& entry : entries) out << ' ' << entry.first << ' ' << entry.second;
            out << '\n';
            continue;
        }
        vector<string> values = getValues(data);
        out << kindOf(data) << ' ' << name << ' ' << values.size();
        for (const string& value : values) out << ' ' << value;
        out << '\n';
    }
    out.close();
    if (!out) throw runtime_error("cannot close file: " + tmp);
    if (rename(tmp.c_str(), path.c_str()) != 0) throw runtime_error("cannot replace data file");
}
void loadData(const string& path, Database& db) {
    ifstream in(path);
    if (!in) return;
    string line;
    size_t lineNumber = 0;
    while (getline(in, line)) {
        lineNumber++;
        if (line.empty()) continue;
        istringstream row(line);
        char type;
        string name;
        size_t n;
        if (!(row >> type >> name) || db.count(name)) throw runtime_error("invalid data file, line " + to_string(lineNumber));
        if (type == 'H'){
            string mode;
            size_t capacity;
            if (!(row >> mode >> capacity >> n) || n > 1000000 || capacity > 1000000 || (mode != "LINEAR" && mode != "DOUBLE")) throw runtime_error("invalid hash table in file");
            HashTable table(capacity, mode == "DOUBLE");
            for (size_t i = 0; i < n; i++){
                string key, value;
                if (!(row >> key >> value)) throw runtime_error("truncated hash table in file");
                table.set(key, value);
            }
            string extra;
            if (row >> extra) throw runtime_error("extra data in file");
            db.emplace(name, std::move(table));
            continue;
        }
        if (type == 'U'){
            size_t capacity;
            if (!(row >> capacity >> n) || capacity == 0 || capacity > 1000000 || n > capacity) throw runtime_error("invalid cache in file");
            LruCache cache(capacity);
            for (size_t i = 0; i < n; i++){
                string key, value;
                if (!(row >> key >> value)) throw runtime_error("truncated cache in file");
                cache.set(key, value);
            }
            string extra;
            if (row >> extra) throw runtime_error("extra data in file");
            db.emplace(name, std::move(cache));
            continue;
        }
        if (!(row >> n) || n > 1000000 || type != 'E') throw runtime_error("invalid data file, line " + to_string(lineNumber));
        Container data = makeContainer(type);
        for (size_t i = 0; i < n; i++) {
            string value;
            if (!(row >> value)) throw runtime_error("truncated data file, line " + to_string(lineNumber));
            get<HashSet>(data).add(value);
        }
        string extra;
        if (row >> extra) throw runtime_error("extra data in file, line " + to_string(lineNumber));
        db.emplace(name, move(data));
    }
}
