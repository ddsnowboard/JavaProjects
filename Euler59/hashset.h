#ifndef HASHSET_H
#define HASHSET_H
#define TRUE 1
#define FALSE 0

struct Link {
    char* key;
    int value;
    struct Link* next;
};

struct HashSet {
    int size;
    struct Link** table;
};

struct HashSet hs_create(int n);

void hs_insert(struct HashSet set, char* s);

int contains(struct HashSet set, char* s);

void hs_free(struct HashSet hs);
#endif
