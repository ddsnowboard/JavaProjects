#include "hashMap.h"
#include <stdio.h>
#include <assert.h>

int main(void) {
    struct hashMap* h = hs_create(40);
    for(long i = 0; i < 60; i++) {
        hs_put(h, i, (void*)(i * i));
    }
    for(long i = 0; i < 60; i++) {
        void** out = hs_get(h, i);
        assert(*out == (void*)(i * i));
        *out = (void*)(i * 2);
    }
    for(long i = 0; i < 60; i++) {
        void** out = hs_get(h, i);
        assert(*out == (void*)(2 * i));
    }
    hs_destroy(h);
}
