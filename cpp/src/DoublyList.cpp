#include "structures/DoublyList.hpp"
#include <stdexcept>

namespace adt {
DoublyList::DoublyList() : head(nullptr), tail(nullptr), len(0) {}
DoublyList::~DoublyList() { clear(); }
void DoublyList::clear() {
    while (head != nullptr) {
        Node* node = head;
        head = head->next;
        delete node;
    }
    tail = nullptr;
    len = 0;
}

DoublyList::DoublyList(DoublyList&& other) noexcept : head(other.head), tail(other.tail), len(other.len) {
    other.head = nullptr;
    other.tail = nullptr;
    other.len = 0;
}

DoublyList& DoublyList::operator=(DoublyList&& other) noexcept {
    if (this == &other) return *this;
    clear();
    head = other.head;
    tail = other.tail;
    len = other.len;
    other.head = nullptr;
    other.tail = nullptr;
    other.len = 0;
    return *this;
}

DoublyList::Node* DoublyList::locate(std::size_t index) const {
    if (index >= len) throw std::out_of_range("index out of range");
    if (index < len / 2) {
        Node* node = head;
        for (std::size_t i = 0; i < index; i++) node = node->next;
        return node;
    }
    Node* node = tail;
    for (std::size_t i = len - 1; i > index; i--) node = node->prev;
    return node;
}

std::size_t DoublyList::size() const { return len; }
void DoublyList::pushFront(const std::string& value) {
    Node* node = new Node{value, nullptr, head};
    if (head != nullptr) head->prev = node;
    else tail = node;
    head = node;
    len++;
}

void DoublyList::pushBack(const std::string& value) {
    if (tail == nullptr) { pushFront(value); return; }
    Node* node = new Node{value, tail, nullptr};
    tail->next = node;
    tail = node;
    len++;
}

void DoublyList::insert(const std::string& where, const std::string& target, const std::string& value){
    if (where == "HEAD") { pushFront(value); return; }
    if (where == "TAIL") { pushBack(value); return; }
    if (where != "BEFORE" && where != "AFTER")
        throw std::invalid_argument("invalid position");
    Node* current = head;
    while (current != nullptr && current->value != target)
        current = current->next;
    if (current == nullptr) throw std::invalid_argument("target not found");
    if (where == "BEFORE"){
        Node* insertion = new Node{value, current->prev, current};
        if (current->prev != nullptr) current->prev->next = insertion;
        else head = insertion;
        current->prev = insertion;
    } else {
        Node* insertion = new Node{value, current, current->next};
        if (current->next != nullptr) current->next->prev = insertion;
        else tail = insertion;
        current->next = insertion;
    }
    len++;
}

std::string DoublyList::unlink(Node* node){
    if (node->prev != nullptr) node->prev->next = node->next;
    else head = node->next;
    if (node->next != nullptr) node->next->prev = node->prev;
    else tail = node->prev;
    std::string value = node->value;
    delete node;
    len--;
    return value;
}

std::string DoublyList::erase(const std::string& where, const std::string& target){
    if (where != "HEAD" && where != "TAIL" && where != "BEFORE" && where != "AFTER")
        throw std::invalid_argument("invalid position");
    if (head == nullptr) throw std::out_of_range("list is empty");
    if (where == "HEAD") return unlink(head);
    if (where == "TAIL") return unlink(tail);
    Node* current = head;
    while (current != nullptr && current->value != target)
        current = current->next;
    if (current == nullptr) throw std::invalid_argument("target not found");
    if (where == "BEFORE"){
        if (current->prev == nullptr)
            throw std::out_of_range("no element before target");
        return unlink(current->prev);
    }
    if (current->next == nullptr)
        throw std::out_of_range("no element after target");
    return unlink(current->next);
}

std::string DoublyList::get(std::size_t index) const { return locate(index)->value; }
std::ptrdiff_t DoublyList::find(const std::string& value) const {
    std::ptrdiff_t index = 0;
    for (Node* node = head; node != nullptr; node = node->next, index++) {
        if (node->value == value) return index;
    }
    return -1;
}

bool DoublyList::remove(const std::string& value){
    Node* current = head;
    while (current != nullptr && current->value != value){
        current = current->next;
    }
    if (current == nullptr) return false;
    unlink(current);
    return true;
}

std::vector<std::string> DoublyList::values(bool reverse) const {
    std::vector<std::string> result;
    if (reverse) {
        for (Node* node = tail; node != nullptr; node = node->prev) result.push_back(node->value);
    } else {
        for (Node* node = head; node != nullptr; node = node->next) result.push_back(node->value);
    }
    return result;
}
}
