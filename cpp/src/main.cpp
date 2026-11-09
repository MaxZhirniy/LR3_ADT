#include "Application.hpp"
#include <iostream>
#include <stdexcept>
#include <string>
using namespace std;

int main(int argc, char* argv[]) {
    string path = "file.data", command;
    bool hasQuery = false;
    try {
        for (int i = 1; i < argc; i++) {
            string key = argv[i];
            if (key == "--file" && i + 1 < argc) path = argv[++i];
            else if (key == "--query" && i + 1 < argc) { command = argv[++i]; hasQuery = true; }
            else if (key == "--help") { cout << "Usage: ./dbms --file file.data [--query 'COMMAND']\n"; return 0; }
            else throw invalid_argument("incorrect command-line argument: " + key);
        }
        Database db;
        loadData(path, db);
        if (hasQuery) return execute(db, path, command);
        cout << "DBMS (C++17). Type HELP for commands, EXIT to finish.\n";
        while (cout << "> " && getline(cin, command)) {
            if (upperText(command) == "EXIT") break;
            execute(db, path, command);
        }
        return 0;
    } catch (const exception& e) {
        cerr << "ERROR: " << e.what() << '\n';
        return 1;
    }
}
