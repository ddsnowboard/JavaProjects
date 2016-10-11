#include <stdio.h>
#include <stdlib.h>
#include "hashTable.h"

int main(int argc, char **argv)
{
    struct ht_Table *table = ht_create(5);
    ht_put(table, 't', 5);
    int *test = ht_get(table, 't');
    if(test != NULL && *test == 5)
    {
        printf("Passed test 1\n");
    }
    else
    {
        printf("Failed the first test!");
        return 1;
    }
    ht_put(table, 'y', 6);
    test = ht_get(table, 't');
    if(test != NULL && *test == 5)
    {
        printf("Passed test 2\n");
    }
    else
    {
        printf("Failed the second test!");
        return 2;
    }
    return 0;
}
