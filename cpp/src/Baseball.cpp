#include "Baseball.hpp"
#include "structures/Stack.hpp"
#include <sstream>
#include <stdexcept>
#include <string>
using namespace std;
using adt::Stack;

static int integer(const string& text){
    size_t used = 0;
    int value = stoll(text, &used);
    if (used != text.size()) throw invalid_argument("invalid integer: " + text);
    return value;
}
int baseball(istringstream& in){
    Stack scores;
    int sum = 0;
    size_t length = 0;
    string action;
    bool any = false;
    while (in >> action){
        any = true;
        if (action == "C"){
            if (length == 0) throw invalid_argument("not enough scores for C");
            sum -= integer(scores.pop());
            length--;
        } else if (action == "D"){
            if (length == 0) throw invalid_argument("not enough scores for D");
            int score = 2 * integer(scores.top());
            scores.push(to_string(score));
            length++;
            sum += score;
        } else if (action == "+"){
            if (length < 2) throw invalid_argument("not enough scores for +");
            string top = scores.pop();
            int score = integer(top) + integer(scores.top());
            scores.push(top);
            scores.push(to_string(score));
            length++;
            sum += score;
        } else {
            int score = integer(action);
            if (score <= 0) throw invalid_argument("score must be positive");
            scores.push(to_string(score));
            length++;
            sum += score;
        }
    }
    if (!any) throw invalid_argument("missing operations");
    return sum;
}
