#include <assert.h>
#include <stdio.h>
#include "hashset.h"
#define HS_SIZE 25

int main(int argc, char** argv) {
    struct HashSet* hs = hs_create(HS_SIZE);
    // The pigeonhole principle! I knew that stuff Fleck taught me would be useful
    for(int i = 0; i < 30 * HS_SIZE + 1; i++) {
        char curr[5];
        sprintf(curr, "%d", i);
        hs_insert(hs, curr);
    }

    for(int i = 0; i < 30 * HS_SIZE + 1; i++) {
        char curr[5];
        sprintf(curr, "%d", i);
        assert(hs_contains(hs, curr));
    }
    hs_print(hs);
    hs_free(hs);
}
