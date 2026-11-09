#include "Huffman.hpp"
#include "structures/Array.hpp"
#include <functional>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using namespace std;
using adt::Array;

static string required(istringstream& in){
    string value;
    if (!(in >> value)) throw invalid_argument("missing command argument");
    return value;
}
static size_t natural(const string& text){
    size_t used = 0;
    unsigned int value = stoull(text, &used);
    if (used != text.size() || text[0] == '-') throw invalid_argument("invalid number: " + text);
    return static_cast<size_t>(value);
}
struct HuffNode {
    unsigned char letter;
    size_t weight;
    int left;
    int right;
};
string huffmanEncode(const string& text){
    if (text.empty()) throw invalid_argument("empty text");
    size_t counts[256] = {};
    for (unsigned char c : text) counts[c]++;
    vector<HuffNode> nodes;
    Array available;
    for (int i = 0; i < 256; i++) if (counts[i]){
        nodes.push_back({static_cast<unsigned char>(i), counts[i], -1, -1});
        available.push(to_string(nodes.size() - 1));
    }
    while (available.size() > 1){
        size_t first = 0, second = 1;
        for (size_t i = 0; i < available.size(); i++){
            size_t weight = nodes[static_cast<size_t>(stoi(available.get(i)))].weight;
            size_t w1 = nodes[static_cast<size_t>(stoi(available.get(first)))].weight;
            if (weight < w1 || (weight == w1 && available.get(i) < available.get(first))) first = i;
        }
        int left = stoi(available.get(first));
        available.erase(first);
        second = 0;
        for (size_t i = 1; i < available.size(); i++){
            size_t weight = nodes[static_cast<size_t>(stoi(available.get(i)))].weight;
            size_t w2 = nodes[static_cast<size_t>(stoi(available.get(second)))].weight;
            if (weight < w2 || (weight == w2 && available.get(i) < available.get(second))) second = i;
        }
        int right = stoi(available.get(second));
        available.erase(second);
        nodes.push_back({0, nodes[left].weight + nodes[right].weight, left, right});
        available.push(to_string(nodes.size() - 1));
    }
    string code[256];
    function<void(int, string)> walk = [&](int id, string bits){
        const HuffNode& node = nodes[id];
        if (node.left == -1){ code[node.letter] = bits.empty() ? "0" : bits; return; }
        walk(node.left, bits + '0');
        walk(node.right, bits + '1');
    };
    walk(stoi(available.get(0)), "");
    string result = "CODE ";
    for (unsigned char c : text) result += code[c];
    result += "\nTABLE";
    for (int i = 0; i < 256; i++) if (!code[i].empty()) result += " " + string(1, static_cast<char>(i)) + ":" + code[i];
    return result;
}
string huffmanDecode(istringstream& in){
    string bits = required(in);
    map<string, char> codes;
    string entry;
    while (in >> entry){
        size_t at = entry.find(':');
        if (at == string::npos) throw invalid_argument("invalid code table");
        size_t symbol = at == 1 ? static_cast<unsigned char>(entry[0]) : natural(entry.substr(0, at));
        string code = entry.substr(at + 1);
        if (symbol > 255 || code.empty() || code.find_first_not_of("01") != string::npos || codes.count(code)) throw invalid_argument("invalid code table");
        codes[code] = static_cast<char>(symbol);
    }
    if (codes.empty() || bits.find_first_not_of("01") != string::npos) throw invalid_argument("invalid bit sequence");
    for (const auto& entry1 : codes) for (const auto& entry2 : codes){
        if (entry1.first != entry2.first && entry1.first.size() < entry2.first.size() && entry2.first.compare(0, entry1.first.size(), entry1.first) == 0) throw invalid_argument("code table is not prefix-free");
    }
    string result, current;
    for (char c : bits){
        current += c;
        map<string, char>::const_iterator it = codes.find(current);
        if (it != codes.end()){ result += it->second; current.clear(); }
    }
    if (!current.empty()) throw invalid_argument("incomplete bit sequence");
    return result;
}
