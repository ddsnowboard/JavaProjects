#include <stdio.h>

void mergeSort(int arr[], int lo, int hi);
void merge(int arr[], int tempArray[], int lo, int hi, int mid);
int main(int argc, char** argv)
{
    int arr[9] = {4, 65, 2, 3, 9, 25, 55, 109, 32};
    size_t size = 9;

    mergeSort(arr, 0, size);

    for(int i = 0; i < size; i++)
        printf("%d ", arr[i]);
    printf("\n");
}

void mergeSort(int arr[], int lo, int hi)
{
    if(hi - lo == 1)
        return;
    int mid = (lo + hi) / 2;
    mergeSort(arr, lo, mid);
    mergeSort(arr, mid, hi);

    int size = hi - lo;
    int tempArray[size];
    merge(arr, tempArray, lo, hi, mid);
    for(int i = 0; i < size; i++)
        arr[i + lo] = tempArray[i];
}

void merge(int arr[], int tempArray[], int lo, int hi, int mid)
{
    int leftLow = lo;
    int rightLow = mid;
    while(leftLow < mid && rightLow < hi)
    {
        if(arr[leftLow] < arr[rightLow])
            *(tempArray++) = arr[leftLow++];
        else
            *(tempArray++) = arr[rightLow++];
    }
    if(leftLow == mid)
    {
        for(; rightLow < hi; rightLow++)
            *(tempArray++) = arr[rightLow];
    }
    else
    {
        for(; leftLow < mid; leftLow++)
            *(tempArray++) = arr[leftLow];
    }
}

