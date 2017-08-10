#include <assert.h>
#include <stdio.h>
#include "hashset.h"
#define HS_SIZE 25

int main(int argc, char** argv) {
    struct HashSet hs = hs_create(HS_SIZE);
    hs_insert(hs, "apples");
    hs_insert(hs, "apples");
    hs_insert(hs, "pears");
    hs_insert(hs, "cider");
    assert(hs_contains(hs, "apples"));
    assert(hs_contains(hs, "apples"));
    assert(hs_contains(hs, "cider"));
    hs_remove(hs, "cider");
    assert(!hs_contains(hs, "cider"));
    assert(!hs_contains(hs, "rocks"));
    hs_free(hs);
}
