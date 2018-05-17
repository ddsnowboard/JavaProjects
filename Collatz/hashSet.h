#ifndef HASHSET_H
#define HASHSET_H
#include <stdlib.h>
struct _node {
    int i;
    struct _node* next;
};

struct hashSet {
    size_t size;
    struct _node** arr;
};

struct hashSet* hs_create(size_t arraySize);
void hs_insert(struct hashSet* hs, int i);
int hs_contains(struct hashSet* hs, int i);
void hs_destroy(struct hashSet* hs);
#endif
