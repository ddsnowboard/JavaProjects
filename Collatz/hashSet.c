#include "hashSet.h"

static void free_chain(struct _node* chain);

struct hashSet* hs_create(size_t arraySize) {
    struct hashSet* out = malloc(sizeof(struct hashSet));
    out->size = arraySize;
    out->arr = malloc(out->size * sizeof(struct _node*));
    return out;
}

void hs_insert(struct hashSet* hs, int i) {
    size_t hashed = i % hs->size;
    struct _node** bin = &hs->arr[hashed];
    while(*bin != NULL) {
        if((*bin)->i == i)
            return;
        bin = &((*bin)->next);
    }
    struct _node* new = malloc(sizeof(struct _node));
    new->i = i;
    new->next = NULL;
    *bin = new;
}

int hs_contains(struct hashSet* hs, int i) {
    size_t hashed = i % hs->size;
    struct _node** bin = &hs->arr[hashed];
    while(*bin != NULL) {
        if((*bin)->i == i)
            return 1;
        bin = &((*bin)->next);
    }
    return 0;
}

void hs_destroy(struct hashSet* hs) {
    for(size_t i = 0; i < hs->size; i++) {
        free_chain(hs->arr[i]);
    }
    free(hs->arr);
    free(hs);
}

static void free_chain(struct _node* chain) {
    if(chain == NULL) {
        return;
    } else {
        free_chain(chain->next);
        free(chain);
    }
}
