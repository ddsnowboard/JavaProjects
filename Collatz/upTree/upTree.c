#include <assert.h>
#include "upTree.h"
struct _upnode {
    long long i;
    struct _upnode* up;
};

static struct _upnode* get_head(struct upTree* ut, long long key);
static struct _upnode* get_node(struct upTree* ut, long long key);
static struct _upnode* get_node_head(struct _upnode* node);

struct upTree* ut_create(size_t array_size) {
    struct upTree* out = malloc(sizeof(struct upTree));
    out->hs = hs_create(array_size);
    return out;
}

void ut_union(struct upTree* ut, long long g1, long long g2) {
    struct _upnode* head1 = get_head(ut, g1);
    struct _upnode* head2 = get_head(ut, g2);
    // I want the group paradigms to be as small as possible
    if(head1->i < head2->i)
        head2->up = head1;
    else
        head1->up = head2;
}

int ut_are_together(struct upTree* ut, long long g1, long long g2) {
    struct _upnode* head1 = get_head(ut, g1);
    struct _upnode* head2 = get_head(ut, g2);
    return head1 == head2;
}

static struct _upnode* get_head(struct upTree* ut, long long key) {
    struct _upnode* startNode = get_node(ut, key);
    return get_node_head(startNode);
}

static struct _upnode* get_node(struct upTree* ut, long long key) {
    struct _upnode** out = (struct _upnode**) hs_get(ut->hs, key);
    if(out == NULL) {
        struct _upnode* new_node = malloc(sizeof(struct _upnode));
        new_node->i = key;
        new_node->up = NULL;
        hs_put(ut->hs, key, new_node);
        out = (struct _upnode**) hs_get(ut->hs, key);
        assert(*out != NULL);
    }
    return *out;
}

static struct _upnode* get_node_head(struct _upnode* node) {
    // This isn't fast yet. We can make it fast pretty easily though.
    if(node->up == NULL) {
        return node;
    } else {
        struct _upnode* out = get_node_head(node->up);
        node->up = out;
        return out;
    }
}

void ut_destroy(struct upTree* ut) {
    hs_free_values(ut->hs);
    hs_destroy(ut->hs);
    free(ut);
}

long long ut_get_group(struct upTree* ut, long long g1) {
    struct _upnode* n = get_head(ut, g1);
    return n->i;
}
