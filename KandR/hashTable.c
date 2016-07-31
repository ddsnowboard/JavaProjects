#include <errno.h>
#include <stdlib.h>
#include "hashTable.h"

static int _hash(struct ht_Table *table, char key);

struct ht_Table *ht_create(int size)
{
    if(size < 0)
    {
        errno = EINVAL;
        return NULL;
    }
    struct ht_Table *out = (struct ht_Table *) malloc(sizeof(struct ht_Table));
    out->table = (struct ht_Node *) malloc(size * sizeof(struct ht_Node));
    out->length = size;
    return out;
}

int *ht_get(struct ht_Table *table, char key)
{
    int idx = _hash(table, key);
    struct ht_Node *node = &(table->table[idx]);
    do
    {
        if(node->key == key)
            return &(node->value);
    } while((node = node->next));
    return NULL;
}

void ht_put(struct ht_Table *table, char key, int value)
{
    int idx = _hash(table, key);
    struct ht_Node *node = &(table->table[idx]);
    do
    {
        if(node->key == key)
        {
            node->value = value;
            return;
        }
    } while(node->next && (node = node->next));
    // If we've gotten here, we need a new node.
    struct ht_Node *newNode = (struct ht_Node *) malloc(sizeof(struct ht_Node));
    newNode->key = key;
    newNode->value = value;
    newNode->next = NULL;
    node->next = newNode;
    return;
}

static int _hash(struct ht_Table *table, char key)
{
    return ((int) key) % table->length;
}
