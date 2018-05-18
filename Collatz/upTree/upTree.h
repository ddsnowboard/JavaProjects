#ifndef UPTREE_H
#define UPTREE_H
#include "../hashMap/hashMap.h"
struct upTree {
    struct hashMap* hs;
};

struct upTree* ut_create(int array_size);
void ut_union(struct upTree* ut, int g1, int g2);
int ut_are_together(struct upTree* ut, int g1, int g2);
int ut_get_group(struct upTree* ut, int g1);
void ut_destroy(struct upTree* ut);
#endif
