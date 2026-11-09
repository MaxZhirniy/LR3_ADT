#include "PatternMatcher.hpp"
#include "structures/Array.hpp"
#include <string>
using namespace std;
using adt::Array;

bool patternMatch(const string& text, const string& pattern){
    Array symbols;
    for (char c : pattern) symbols.push(string(1, c));
    size_t i = 0, j = 0;
    size_t lastStar = symbols.size(), matched = 0;
    while (i < text.size()){
        if (j < symbols.size() && (symbols.get(j) == "?" || symbols.get(j) == string(1, text[i]))){ i++; j++; }
        else if (j < symbols.size() && symbols.get(j) == "*"){ lastStar = j++; matched = i; }
        else if (lastStar < symbols.size()){ j = lastStar + 1; i = ++matched; }
        else return false;
    }
    while (j < symbols.size() && symbols.get(j) == "*") j++;
    return j == symbols.size();
}
