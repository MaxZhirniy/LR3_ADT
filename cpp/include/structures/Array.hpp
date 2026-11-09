#pragma once
#include <cstddef>
#include <vector>
#include <string>
namespace adt {
class Array {
    std::string* data;
    std::size_t len;
    std::size_t cap;
    void reserve(std::size_t capacity);
public:
    Array();
    ~Array();
    Array(const Array&) = delete;
    Array& operator=(const Array&) = delete;
    Array(Array&& other) noexcept;
    Array& operator=(Array&& other) noexcept;

    std::size_t size() const;
    void insert(std::size_t index, const std::string& value);
    void push(const std::string& value);
    std::string get(std::size_t index) const;
    void set(std::size_t index, const std::string& value);
    std::string erase(std::size_t index);
    std::vector<std::string> values() const;
};
}
