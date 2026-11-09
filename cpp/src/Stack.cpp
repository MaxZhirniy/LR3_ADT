#include "structures/Stack.hpp"
#include <stdexcept>

namespace adt {
Stack::Stack() : topNode(nullptr) {}
Stack::~Stack() { clear(); }
void Stack::clear() {
    while (topNode != nullptr) {
        Node* node = topNode;
        topNode = topNode->next;
        delete node;
    }
}

Stack::Stack(Stack&& other) noexcept : topNode(other.topNode) { other.topNode = nullptr; }
Stack& Stack::operator=(Stack&& other) noexcept {
    if (this == &other) return *this;
    clear();
    topNode = other.topNode;
    other.topNode = nullptr;
    return *this;
}

void Stack::push(const std::string& value) { topNode = new Node{value, topNode}; }
std::string Stack::pop() {
    if (topNode == nullptr) throw std::out_of_range("stack is empty");
    Node* node = topNode;
    std::string value = node->value;
    topNode = node->next;
    delete node;
    return value;
}

std::string Stack::top() const {
    if (topNode == nullptr) throw std::out_of_range("stack is empty");
    return topNode->value;
}

std::vector<std::string> Stack::values() const {
    std::vector<std::string> result;
    for (Node* node = topNode; node != nullptr; node = node->next) result.push_back(node->value);
    return result;
}
}
