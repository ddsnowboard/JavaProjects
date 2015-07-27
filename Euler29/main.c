#include <stdio.h>
#include <math.h>
#define MAX 10000

int search(int[], int, int);

int main(int argc, char** argv)
{
    int nums[MAX];
    for(int i = 0; i < MAX; i++)
    {
        nums[i] = 0;
    }
    int currLen = 0;
    unsigned long long currNum;
    for(unsigned long long a = 2; a <= 100; a++)
    {
        for(unsigned long long b = 2; b <= 100; b++)
        {
            currNum = pow(a, b);
            printf("a is %llu and b is %llu and a^b is %llu\n", a, b, currNum);
            if(search(nums, currLen, currNum) == 0)
            {
                nums[currLen++] = currNum;
            }
        }
    }
    printf("%d\n", currLen);
}

int search(int *arr, int len, int i)
{
    for(int j = 0; j < len; j++)
    {
        if(arr[j] == i)
        {
            return 1;
        }
    }
    return 0;
}
