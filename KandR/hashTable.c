#include <stdlib.h>
#include "hashTable.h"

HashTable ht_create(int size)
{
    HashTable out = (HashTable) malloc(size * sizeof(struct Node *));
    return out;
}
