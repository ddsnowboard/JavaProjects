// Evidently this way won't actually work. It is too dependent on 
// order, which isn't good. I don't know what to do though. I have this idea that 
// I could do something where I start by building all the way up to the target
// using the smallest denomination, and then I go through the array of 1's and 
// see if all the subarrays can be replaced by a single other coin. I don't really 
// know what I'd do past that though. 
//
// Another option would be to keep doing what I was doing but instead of 
// having a set of linked lists, I would create a python counter, basically, 
// and put those in a set. I don't know what I'll do, but I bet that could work. 
// Slowly

#include <cstdio>
#include <iostream>

#define NUMBER_OF_DENOMINATIONS 8
#define STARTING_AMOUNT 200
using namespace std;

void printArray(int arr[STARTING_AMOUNT + 1]);

int main(int argc, char** argv)
{
    const int DENOMINATIONS[NUMBER_OF_DENOMINATIONS] = {1,2,5,10,20, 50, 100, 200};
    int ways[STARTING_AMOUNT + 1];
    ways[0] = 1;
    for(int i = 1; i <= STARTING_AMOUNT; i++)
        ways[i] = 0;

    for(int i = 0; i < NUMBER_OF_DENOMINATIONS; i++)
    {
        int coinAmount = DENOMINATIONS[i];
        cout << "coinAmount is " << coinAmount << "\n";
        for(int j = coinAmount; j <= STARTING_AMOUNT; j++)
        {
            if(j <= STARTING_AMOUNT && j - coinAmount >= 0)
                ways[j] += ways[j - coinAmount];
        }
        printArray(ways);
    }
    cout << "The answer is " << ways[STARTING_AMOUNT] << "\n";
}

void printArray(int arr[STARTING_AMOUNT + 1])
{
    for(int i = 0; i <= STARTING_AMOUNT; i++)
        cout << arr[i] << "\n";
    cout << "\n\n\n\n";
}
