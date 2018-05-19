#include <stdio.h>
#include "upTree/upTree.h"
#define MAX 1000000
#define SET_SIZE 3000000

struct upTree* ut;

int collatz(long long);

int main(int argc, char** argv)
{
    ut = ut_create(SET_SIZE);
    for(int i = 1; i < MAX; i++)
    {
        int curr;
        if((curr = collatz(i)) != 1) {
            printf("%d returned %d\n", i, curr);
        }
    }
    ut_destroy(ut);
    return 0;
}

// Returns 0 if Collatz Conjecture works, a number if otherwise.
int collatz(long long i)
{
    if(i == 1)
        return 1;
    else if(i < 0) {
        return -1;
    }  else if(ut_get_group(ut, i) != i) {
        return ut_get_group(ut, i); 
    } else {
        int endpoint;
        if(i % 2 == 0) {
            endpoint = collatz(i / 2);
        } else {
            endpoint = collatz(3 * i + 1);
        }  
        ut_union(ut, i, endpoint);
        return endpoint;
    }
}
