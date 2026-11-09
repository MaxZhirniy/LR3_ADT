#pragma once
#include "Commands.hpp"
#include <string>

int execute(Database& db, const std::string& path, const std::string& command);
