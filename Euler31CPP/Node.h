class Node {
    public:
        int value;
        Node *next;
        bool operator==(const Node &other) const
        {
            return value == other.value && next == other.next;
        }
};

