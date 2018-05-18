#include "hashMap.h"

struct _node {
    int key;
    void* val;
    struct _node* next;
};

static void free_chain(struct _node* chain);

static struct _node** findNode(struct _node** chainStart, int key);

static size_t hash(struct hashMap* hs, int i);

struct hashMap* hs_create(size_t arraySize) {
    struct hashMap* out = malloc(sizeof(struct hashMap));
    out->size = arraySize;
    out->arr = calloc(out->size, sizeof(struct _node*));
    return out;
}

void hs_put(struct hashMap* hs, int key, void* val) {
    size_t hashed = hash(hs, key);
    struct _node** bin = findNode(&hs->arr[hashed], key);
    if(*bin == NULL) {
        struct _node* new = malloc(sizeof(struct _node));
        new->key = key;
        new->next = NULL;
        *bin = new;
    }
    (*bin)->val = val;
}

int hs_contains(struct hashMap* hs, int key) {
    return hs_get(hs, key) != NULL;
}

void** hs_get(struct hashMap* hs, int key) {
    size_t hashed = hash(hs, key);
    struct _node** bin = findNode(&hs->arr[hashed], key);
    if(*bin == NULL)
        return NULL;
    else
        return &(*bin)->val;
    return 0;
}

void hs_destroy(struct hashMap* hs) {
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

static struct _node** findNode(struct _node** chainStart, int key) {
    if(*chainStart == NULL || (*chainStart)->key == key)
        return chainStart;
    else
        return findNode(&((*chainStart)->next), key);
}

static size_t hash(struct hashMap* hs, int i) {
    return i % hs->size;
}

void hs_free_values(struct hashMap* hs) {
    for(int i = 0; i < hs->size; i++) {
        struct _node* n = hs->arr[i];
        while(n != NULL) {
            free(n->val);
            n = n->next;
        }
    }
}
