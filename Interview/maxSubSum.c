#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

int sum(int *tortoise, int *hare);

int main(int argc, char **argv)
{
    // This problem gives you an array of positive
    // and negative numbers and asks for the
    // biggest sub-array of it. I think I can use a 
    // tortoise and hare algorithm of some sort.
    //
    //
    // This implementation works I think, but it's not slick. 
    // I should probably improve it.
    int INPUT[] = {1, -2, 3, 10, -4, 7, 2, -5};
    const int LENGTH = 8;
    int *tortoise = INPUT;
    int *hare = INPUT + 1;
    int biggestSum = sum(tortoise, hare);
    while(tortoise < INPUT + LENGTH)
    {
        while(hare < INPUT + LENGTH - 1)
        {
            int total = sum(tortoise, hare);
            printf("Tortoise is %ld, and hare is %ld, and sum is %d\n", tortoise - INPUT, hare - INPUT, total);
            if(total > biggestSum)
                biggestSum = total;
            else if(hare[1] > total && hare[1] > biggestSum)
            {
                printf("Short-circuited! Tortoise is %ld, hare is %ld for %d, and sum is %d\n", tortoise - INPUT, hare - INPUT + 1, hare[1], sum(tortoise, hare));
                // We're going to increment tortoise outside the loop
                tortoise = hare - 1;
                biggestSum = *hare;
                break;
            }
            hare++;
        }
        tortoise++;
        hare = tortoise + 1;
    }

    printf("the output was %d\n", biggestSum);
}


int sum(int *tortoise, int *hare)
{
    int out = 0;
    do
    {
        out += *(tortoise++);
    } while(tortoise <= hare);
    return out;
}
