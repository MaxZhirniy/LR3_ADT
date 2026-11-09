#pragma once
#include <sstream>
#include <string>
std::string huffmanEncode(const std::string& text);
std::string huffmanDecode(std::istringstream& in);
