struct ht_Node {
    char key;
    int value;
    struct ht_Node *next;
};

struct ht_Table {
    size_t length;
    struct ht_Node *table;
};

struct ht_Table *ht_create(int size);

int *ht_get(struct ht_Table *table, char key);

void ht_put(struct ht_Table *table, char key, int value);
