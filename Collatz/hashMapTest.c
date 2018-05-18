#include "hashMap.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    struct hashMap* h = hs_create(40);
    for(int i = 0; i < 60; i++) {
        hs_put(h, i, i * i);
    }
    for(int i = 0; i < 60; i++) {
        int* out = hs_get(h, i);
        assert(*out == i * i);
        *out = i * 2;
    }
    for(int i = 0; i < 60; i++) {
        int* out = hs_get(h, i);
        assert(*out == 2 * i);
    }
    hs_destroy(h);
}
