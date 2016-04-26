struct HT_Node {
    char key;
    int value;
    struct HT_Node *next;
};

typedef struct HT_Node **HashTable;

HashTable ht_create(int size);

int *ht_get(HashTable table, char key);

void ht_put(HashTable table, char key, int value);
