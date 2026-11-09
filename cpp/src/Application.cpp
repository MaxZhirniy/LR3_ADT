#include "Application.hpp"
#include <iostream>
#include <stdexcept>
using namespace std;

int execute(Database& db, const string& path, const string& command) {
    try {
        bool changed = false;
        string result = runCommand(db, command, changed);
        if (changed) saveData(path, db);
        if (!result.empty()) cout << result << '\n';
        return 0;
    } catch (const exception& e) {
        cerr << "ERROR: " << e.what() << '\n';
        return 1;
    }
}
