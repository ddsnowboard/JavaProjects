#include <stdio.h>
#define MAX 1000

int search(int*, int);
int pow(int, int);

int main(int argc, char** argv)
{
    int nums[MAX];
    int currLen = 0;
    int currNum;
    for(int a = 2; a <= 100; a++)
    {
        for(int b = 2; b <= 100; b++)
        {
            currNum = pow(a, b)
            if(!search(nums*, currLen, currNum))
            {
                nums[currLen] = currNum;
                currLen++;
            }
        }
    }
    printf("%d", currLen);
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

int pow(int a, int b)
{
    for(int i = 0; i < b; i++)
    {
        a *= a;
    }
    return a;
}
