#ifndef UPTREE_H
#define UPTREE_H
#include "../hashMap/hashMap.h"
struct upTree {
    struct hashMap* hs;
};

struct upTree* ut_create(size_t array_size);
void ut_union(struct upTree* ut, long long g1, long long g2);
int ut_are_together(struct upTree* ut, long long g1, long long g2);
long long ut_get_group(struct upTree* ut, long long g1);
void ut_destroy(struct upTree* ut);
#endif
