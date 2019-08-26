#include "Graph.cpp"
#include <fstream>
#include <iostream>
#include <regex>
using std::cout;
using std::endl;
using std::string;
using std::vector;

Graph readInput();

void print(const vector<string>& s);
vector<string> topologicalSort(Graph g);

int main() {
    Graph g = readInput();
    auto tSort = topologicalSort(g);
    print(tSort);
}

Graph readInput() {
    Graph out;

    std::ifstream f("input.txt");
    string line;
    std::regex r("Step ([A-Z]) must be finished before step ([A-Z]) can begin");
    std::smatch match;
    while(!f.eof()) {
        std::getline(f, line);
        if(std::regex_search(line, match, r)) {
            auto start = match[1].str();
            auto end = match[2].str();
            out.addEdge(start, end);
        }  
    }
    return out;
}

vector<string> topologicalSort(Graph g) {
    vector<string> out;
    vector<string> sources;
    while((sources = g.allSources()).size() != 0) {
        string ourSource = *std::min_element(sources.begin(), sources.end());
        out.push_back(ourSource);
        g.removeNode(ourSource);
    }
    return out;
}

void print(const vector<string>& s) {
    for(const auto& ss : s) {
        cout << ss;
    }
    cout << endl;
}
