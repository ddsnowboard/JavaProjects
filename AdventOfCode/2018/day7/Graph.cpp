#ifndef GRAPH_CPP
#define GRAPH_CPP
#include <string>
#include <unordered_map>
#include <algorithm>
#include <vector>

class Graph {
    private:
        class Node {
            public:
                std::vector<std::string> adjacents;
                std::string name;

                Node(const std::string& name) : name(name) {
                }
                bool operator==(const Node& other) const {
                    return other.name == name;
                }
                bool operator!=(const Node& other) const {
                    return !(other == *this);
                }
        };

        std::unordered_map<std::string, Node> nodes;

        void ensureNodeExists(const std::string& name) {
            if(!hasNode(name))
                nodes.emplace(name, name);
        }

    public: 
        bool hasNode(const std::string& name) const {
            return nodes.count(name) == 1;
        }

        void addEdge(const std::string& a, const std::string& b) {
            ensureNodeExists(a);
            ensureNodeExists(b);
            Node& s = nodes.at(a);
            s.adjacents.push_back(b);
        }

        bool hasEdge(const std::string& a, const std::string& b) const {
            const Node& nodeA = nodes.at(a);
            return hasNode(a) && hasNode(b) && std::find(nodeA.adjacents.cbegin(), nodeA.adjacents.cend(), b) != nodeA.adjacents.end();
        }

        bool isSource(const std::string& a) const {
            for(const auto& p : nodes) {
                if(p.first != a && hasEdge(p.first, a)) {
                    return false;
                }
            }
            return true;
        }

        std::vector<std::string> allSources() const {
            std::vector<std::string> out;
            for(const auto& p : nodes) {
                if(isSource(p.first))
                    out.push_back(p.first);
            }
            return out;
        }

        void removeNode(std::string n) {
            nodes.erase(n);
            for(auto& p : nodes) {
                Node& node = p.second;
                auto it = std::find(node.adjacents.begin(), node.adjacents.end(), n);
                if(it != node.adjacents.end()) {
                    node.adjacents.erase(it, it);
                }
            }
        }

        std::vector<std::string> getNodes() const {
            std::vector<std::string> out(nodes.size());
            std::transform(nodes.begin(), nodes.end(), out.begin(), [](const std::pair<std::string, Graph::Node>& p) { 
                    return p.first;
                    });
            return out;
        }

        const std::vector<std::string>& getAdjacents(const std::string& s) const {
            std::vector<std::string> out;
            const Node& n = nodes.at(s);
            return n.adjacents;
        }

        bool reaches(const std::string& a, const std::string& b) const {
            if(a == b) {
                return true;
            } else {
                // I'm lazy and assuming this is a DAG
                for(const auto& other : nodes.at(a).adjacents) {
                    if(reaches(other, b)) {
                        return true;
                    }
                }
                return false;
            }
        }
};
#endif
