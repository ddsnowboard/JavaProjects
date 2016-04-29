#include <errno.h>
#include <stdlib.h>
#include "hashTable.h"

int _hash(struct ht_Table *table, char key);

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
    struct ht_Node node = table->table[idx];
    // This isn't done yet. I'm not really in a good spot to do it though.
    while(node->next)
    {
        node = node->next;
    }
}

void ht_put(struct ht_Table *table, char key, int value)
{

}

int _hash(struct ht_Table *table, char key)
{
    return ((int) key) % table->length;
}
