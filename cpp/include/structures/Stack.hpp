#pragma once
#include <vector>
#include <string>

namespace adt {
class Stack {
    struct Node {
        std::string value;
        Node* next;
    };
    Node* topNode;
    void clear();
public:
    Stack();
    ~Stack();
    Stack(const Stack&) = delete;
    Stack& operator=(const Stack&) = delete;
    Stack(Stack&& other) noexcept;
    Stack& operator=(Stack&& other) noexcept;

    void push(const std::string& value);
    std::string pop();
    std::string top() const;
    std::vector<std::string> values() const;
};
}
