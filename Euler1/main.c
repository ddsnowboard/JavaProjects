#include <stdio.h>
#define TOP 1000

int main(int argc, char **argv)
{
    int i = 0;
    int sum = 0;
    for(; i < TOP; ++i)
    {
        if(i % 3 == 0 || i % 5 == 0)
            sum += i;
    }
    printf("The sum was %d\n", sum);
    return 0;
}
