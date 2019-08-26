#include <utility>
#include <vector>
#include <string>
#include <iostream>
#include <fstream>
#include <regex>
#include <algorithm>
#include <queue>
#include <assert.h>
using std::endl;
using std::cout;
using std::pair;
using std::vector;
using std::string;

typedef pair<int, int> Coordinate;
struct Point {
    const int LARGE_NUMBER = 100000;
    const Coordinate* parent;
    bool marked;
    int count = 0;
    vector<int> distances;
    int smallestDistance() const {
        if(this->distances.size() == 0)
            return LARGE_NUMBER;
        else 
            return *std::min_element(this->distances.begin(), this->distances.end());
    }
    int sumDistances() const {
        return std::accumulate(this->distances.begin(), this->distances.end(), 0);
    }
};

struct Board {
    std::map<Coordinate, Point> board;
    int maxX;
    int maxY; 
    int minX; 
    int minY;
};

void marking_bfs(const Coordinate& coord, Board& board);
vector<Coordinate> get_input();
void print_board(const vector<Coordinate> starts, const Board& board);
int distance(const Coordinate& a, const Coordinate& b);
vector<const Coordinate*> filter_elements_by_total_distance_less_than(const Board board, int maximum);

int main(int argc, char** argv) {
    (void)argc;
    (void)argv;
    const vector<Coordinate> coords = get_input();
    Board board;
    {
        int maxX = (*std::max_element(coords.begin(), coords.end(), [](const Coordinate& c1, const Coordinate& c2) { return c1.first < c2.first; })).first;
        int maxY = (*std::max_element(coords.begin(), coords.end(), [](const Coordinate& c1, const Coordinate& c2) { return c1.second < c2.second; })).second;
        int minX = (*std::min_element(coords.begin(), coords.end(), [](const Coordinate& c1, const Coordinate& c2) { return c1.first < c2.first; })).first;
        int minY = (*std::min_element(coords.begin(), coords.end(), [](const Coordinate& c1, const Coordinate& c2) { return c1.second < c2.second; })).second;

        board.maxX = maxX;
        board.maxY = maxY;
        board.minX = minX;
        board.minY = minY;
    }

    for(auto& c : coords) {
        marking_bfs(c, board);
    }

    const Coordinate& nearestTopLeft = *std::min_element(coords.begin(), coords.end(), [&](const Coordinate& c1, const Coordinate& c2) {
            Coordinate topLeft = std::make_pair(board.minX, board.minY);
            return distance(c1, topLeft) < distance(c2, topLeft);
            });

    const Coordinate& nearestTopRight = *std::min_element(coords.begin(), coords.end(), [&](const Coordinate& c1, const Coordinate& c2) { 
            Coordinate topRight = std::make_pair(board.maxX, board.minY);
            return distance(c1, topRight) < distance(c2, topRight);
            });

    const Coordinate& nearestBottomLeft = *std::min_element(coords.begin(), coords.end(), [&](const Coordinate& c1, const Coordinate& c2) { 
            Coordinate bottomLeft = std::make_pair(board.minX, board.maxY);
            return distance(c1, bottomLeft) < distance(c2, bottomLeft);
            });

    const Coordinate& nearestBottomRight = *std::min_element(coords.begin(), coords.end(), [&](const Coordinate& c1, const Coordinate& c2) { 
            Coordinate bottomRight = std::make_pair(board.maxX, board.maxY);
            return distance(c1, bottomRight) < distance(c2, bottomRight);
            });
    // Count up all the points that belong to each coordinate, but don't count the ones that go to one of the above 4, or any of the actual coordinates.
    for(int x = board.minX; x <= board.maxX; x++) {
        for(int y = board.minY; y <= board.maxY; y++) {
            Coordinate c = std::make_pair(x, y);
            const auto& point = board.board[c];
            if(point.parent != nullptr) {
                Point& parentPoint = board.board[*(point.parent)];
                parentPoint.count++;
            }
        }
    }

    {
        // Zero out the corner points because they'll have infinite points
        const Coordinate* corners[] = {&nearestBottomRight, &nearestTopRight, &nearestBottomLeft, &nearestTopLeft};
        for(const auto& c : corners) {
            board.board[*c].count = 0;
        }
    }

    auto& finalCoord = *std::max_element(coords.begin(), coords.end(), [&](const Coordinate& c1, const Coordinate& c2) {
            return board.board[c1].count < board.board[c2].count;
            });

    cout << "Part 1: " << board.board[finalCoord].count << endl;
    cout << "Part 2: " << filter_elements_by_total_distance_less_than(board, 10000).size() << endl;
}


vector<Coordinate> get_input() {
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

bool on_board(const Coordinate& coord, const Board& board) {
    return coord.first >= board.minX && coord.first <= board.maxX &&
        coord.second >= board.minY && coord.second <= board.maxY;
}

void unmark_board(Board& board) {
    for(auto& p : board.board) {
        p.second.marked = false;
    }
}

void marking_bfs(const Coordinate& coord, Board& board) {
    std::queue<std::pair<Coordinate, int>> q;
    q.push(std::make_pair(coord, 0));
    unmark_board(board);
    board.board[coord].marked = true;
    while(q.size() > 0) {
        // Read point
        auto& currPair = q.front();
        auto& currCoord = currPair.first;
        int currentLayer = currPair.second;
        assert(board.board[currCoord].marked);

        // Add neighbors
        int x = currCoord.first;
        int y = currCoord.second;
        Coordinate nextCoords[] = {{x, y - 1},
            {x + 1, y}, {x - 1, y}, {x, y + 1}};
        for(auto& nextCoord : nextCoords) {
            if(on_board(nextCoord, board) && !board.board[nextCoord].marked) {
                q.push(std::make_pair(nextCoord, currentLayer + 1));
                board.board[nextCoord].marked = true;
            }
        }

        // Mark distance
        Point& currPoint = board.board[currCoord];
        if(currPoint.smallestDistance() > currentLayer) {
            currPoint.parent = &coord;
        } else if (currPoint.smallestDistance() == currentLayer) {
            currPoint.parent = nullptr;
        }
        currPoint.distances.push_back(currentLayer);
        q.pop();
    }
}

void print_board(const vector<Coordinate> starts, const Board& board) {
    for(int y = board.minY; y <= board.maxY; y++) {
        for(int x = board.minX; x <= board.maxX; x++) {
            const auto parent = board.board.at(std::make_pair(x, y)).parent;
            if(parent == nullptr) {
                cout << ".";
            } else {
                size_t distance = std::find(starts.begin(), starts.end(), *parent) - starts.begin();
                char name = 'A' + distance;
                cout << name;
            }
        }
        cout << endl;
    }
}

int distance(const Coordinate& a, const Coordinate& b) {
    return abs(a.first - b.first) + abs(a.second - b.second);
}

vector<const Coordinate*> filter_elements_by_total_distance_less_than(const Board board, int maximum) {
    vector<const Coordinate*> out;
    for(int x = board.minX; x <= board.maxX; x++) {
        for(int y = board.minY; y <= board.maxY; y++) {
            const Coordinate c = std::make_pair(x, y);
            const Point p = board.board.at(c);
            if(p.sumDistances() < maximum) {
                out.push_back(&c);
            }
        }
    }
    return out;
}
