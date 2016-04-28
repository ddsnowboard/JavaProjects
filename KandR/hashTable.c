#include <errno.h>
#include <stdlib.h>
#include "hashTable.h"

int _hash(HashTable table, char key);

HashTable ht_create(int size)
{
    if(size < 0)
    {
        errno = EINVAL;
        return NULL;
    }
    HashTable out = (HashTable) malloc(size * sizeof(struct Node *));
    return out;
}

int *ht_get(HashTable table, char key)
{

}

int _hash(HashTable table, char key)
{
    // I've got to find out a way to hold the size of the hashtable 
    // so I can mod by it here. I don't know how to do that though. 
    // Null terminator or something?
    return ((int) key) % 
}
