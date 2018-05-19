#ifndef HASMMAP_H
#define HASMMAP_H
#include <stdlib.h>
struct hashMap {
    size_t size;
    struct _node** arr;
};

struct hashMap* hs_create(size_t arraySize);
void hs_put(struct hashMap* hs, long long key, void* val);
int hs_contains(struct hashMap* hs, long long key);
void** hs_get(struct hashMap* hs, long long key);
void hs_destroy(struct hashMap* hs);
void hs_free_values(struct hashMap* hs);
#endif
