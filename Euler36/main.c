// https://projecteuler.net/problem=36
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
char* reverse(char* s);
char* itoa(int i, int base);
int palindrome(char* a);

int main(int argc, char** argv) 
{
    int tot = 0;
    // char* currDec;
    // char* currBin;
    for(int i = 1; i <= 1000000; ++i)
    {
        if(palindrome(itoa(i, 10)) == 0 && palindrome(itoa(i, 2)) == 0)
        {
            tot += i;
        }
    }
    printf("Total is %d\n", tot);
}
char* reverse(char* s)
{
    int len = strlen(s);
    char* out = malloc(len + 1);
    int j = 0;
    for(int i = len - 1; i >= 0; --i, ++j)
    {
        out[j] = s[i];
    }
    out[j + 1] = '\0';
    return out;
}
char* itoa(int i, int base)
{
    int sign = i > 0;
    if(!sign)
        i *= -1;
    int exp = base;
    int idx = 0;
    char* out = malloc(20);
    while(i % (exp / base) != i)
    {
        out[idx] = (i % exp) / (exp / base) + '0';
        exp *= base;
        idx++;
    }
    if(!sign)
        out[idx++] = '-';
    out[idx] = '\0';
    return reverse(out);
}
int palindrome(char* a)
{
    size_t len = strlen(a);
    for(int i = 0, j = len - 1; i <= len / 2; ++i, --j)
    {
        if(a[i] != a[j])
            return 1;
    }
    return 0;
}
