#include <utility>
#include <vector>
#include <string>
#include <iostream>
using std::endl;
using std::cout;
using std::pair;
using std::vector;
using std::string;

typedef pair<int, int> Coordinate

vector<Coordinate> getInput();
int main(int argc, char** argv) {
    const vector<Coordinate> coords = getInput();
}


vector<Coordinate> getInput() {
    vector<Coordinate> out();
    ifstream f("input.txt");
    string line();
    while(getline(f, line)) {
        // Parse into out.push_back()
    }
    return out;
}
