#pragma once
#include "Database.hpp"
#include <string>

std::string runCommand(Database& db, const std::string& command, bool& changed);
std::string upperText(std::string text);
