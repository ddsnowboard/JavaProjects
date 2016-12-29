#include <functional>
#include "Node.h"
namespace std {
    template<>
        struct hash<Node>
        {
            std::size_t operator()(const Node& node) const {
                using std::size_t;
                using std::hash;
                return hash<int>()(node.value) << 1 ^ (node.next != NULL ? hash<Node>()(*node.next) : 0);
            }
        };
}
