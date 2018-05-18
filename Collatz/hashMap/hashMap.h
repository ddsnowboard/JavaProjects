#ifndef HASMMAP_H
#define HASMMAP_H
#include <stdlib.h>
struct hashMap {
    size_t size;
    struct _node** arr;
};

struct hashMap* hs_create(size_t arraySize);
void hs_put(struct hashMap* hs, int key, int val);
int hs_contains(struct hashMap* hs, int key);
int* hs_get(struct hashMap* hs, int key);
void hs_destroy(struct hashMap* hs);
#endif
