#include <stdlib.h>
#include <string.h>
#include <math.h>

#include "hashset.h"

int _hash(char* s) {
    int idx = 0;
    int length = strlen(s);
    int out = 0;
    char curr;
    while((curr = s[idx++]))
        out += (int) curr * pow(31, length - 1 - idx);
    return out;
}

struct HashSet hs_create(int n) {
    struct HashSet out;
    // calloc ensures these are all null
    out.table = calloc(n, sizeof(struct Link *));
    out.size = n;
    return out;
}

void hs_insert(struct HashSet set, char* s) {
    int hashCode = _hash(s) / set.size;
    struct Link** walker = &set.table[hashCode];
    while(*walker != NULL) {
        struct Link curr = **walker;
        if(strcmp(curr.key, s) == 0)
            return;
        walker = &(**walker).next;
    }
    struct Link* newLink = malloc(sizeof(struct Link));
    char* newKey = malloc(sizeof(char) * strlen(s) + 1);
    strcpy(newKey, s);
    newLink->key = newKey;
    newLink->value = TRUE;
    newLink->next = NULL;
    *walker = newLink;
}

int contains(struct HashSet set, char* s) {
    int hashCode = _hash(s) / set.size;
    struct Link* walker = set.table[hashCode];
    while(walker != NULL) {
        struct Link curr = *walker;
        if(strcmp(curr.key, s) == 0)
            return TRUE;
        walker = walker->next;
    }
    return FALSE;
}

static void _freeChain(struct Link* l) {
    if(l == NULL)
        return;
    _freeChain(l->next);
    free(l->key);
    free(l);
}

void hs_free(struct HashSet hs) {
    for(int i = 0; i < hs.size; i++) {
        if(hs.table[i] != NULL)
            _freeChain(hs.table[i]);
    }
    free(hs.table);
}

void hs_remove(struct HashSet hs, char* s) {
    int hashCode = _hash(s) / hs.size;
    struct Link** walker = &hs.table[hashCode];
    while(*walker != NULL) {
        struct Link curr = **walker;
        if(strcmp(curr.key, s) == 0) {
            struct Link* newNext = curr.next;
            (**walker).next = NULL;
            _freeChain(*walker);
            *walker = newNext;
            return;
        }
        walker = &(*walker)->next;
    }
}
