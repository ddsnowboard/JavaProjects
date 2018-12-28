#include <utility>
#include <vector>
#include <string>
#include <iostream>
#include <fstream>
#include <regex>
#include <algorithm>
using std::endl;
using std::cout;
using std::pair;
using std::vector;
using std::string;

typedef pair<int, int> Coordinate;

vector<Coordinate> getInput();
int main(int argc, char** argv) {
    const vector<Coordinate> coords = getInput();
    int maxX = (*std::max_element(coords.begin(), coords.end(), [](const Coordinate& c1, const Coordinate& c2) { return c1.first < c2.first; })).first;
    // int maxY = std::max(coords, [](const Coordinate& c1, const Coordinate& c2) { return c1.first < c2.first; });
    // int minX = std::min(coords, [](const Coordinate& c1, const Coordinate& c2) { return c1.first < c2.first; });
    // int minY = std::min(coords, [](const Coordinate& c1, const Coordinate& c2) { return c1.first < c2.first; });
}


vector<Coordinate> getInput() {
    vector<Coordinate> out;
    std::ifstream f("input.txt");
    string line;
    std::regex r("(\\d+), (\\d+)");
    std::smatch match;
    while(!f.eof()) {
        std::getline(f, line);
        if(std::regex_search(line, match, r)) {
            int x = std::stoi(match[1].str());
            int y = std::stoi(match[2].str());
            out.push_back(std::make_pair(x, y));
        }  
    }
    return out;
}
