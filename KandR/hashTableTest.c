#include <stdio.h>
#include <stdlib.h>
#include "hashTable.h"

int main(int argc, char **argv)
{
    struct ht_Table *table = ht_create(5);
    ht_put(table, 't', 5);
    printf("%d\n", *ht_get(table, 't'));
    ht_put(table, 'y', 6);
    printf("%d\n", *ht_get(table, 'y'));
    return 0;
}
