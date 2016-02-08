#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>
#define LENGTH 2

int canCancelDigits(int top, int bottom);
char* itoa(int i, int base);
char* removeChar(char* s, char c); 
char* reverse(char* s);


int main(int argc, char** argv)
{
    int i = 10;
    for(; i < 100; ++i)
    {
        int j = 10;
        for(; j < 100; ++j)
        {
            if(canCancelDigits(i, j))
            {
                if(!(i == j || i / j == 10 || j / i == 10))
                {
                    printf("%d / %d is one.\n", i, j);
                }
            }
        }
    }
    return 0;
}

int canCancelDigits(int top, int bottom)
{
    char* sTop = itoa(top, 10);
    char* sBottom = itoa(bottom, 10);
    int cancelled = 0;
    if(!(strlen(sTop) == LENGTH && strlen(sBottom) == LENGTH))
        return -1;
    else
    {
        int i = 0;
        for(; i < 2; ++i)
        {
            char* idx = index(sBottom, sTop[i]);
            if(!(idx == NULL || idx[0] == '0'))
            {
                sTop = removeChar(sTop, *idx);
                sBottom = removeChar(sBottom, *idx);
                cancelled = 1;
                break;
            }
        }
    }
    if(cancelled)
    {
        float fTop = atof(sTop);
        float fBottom = atof(sBottom);
        if(fTop / fBottom == (float)top / (float)bottom)
            return 1;
    }
    return 0;
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

char* removeChar(char* s, char c)
{
    int i = 0; 
    char currChar;
    int currLen = 0;
    char* out = malloc(strlen(s));
    while((currChar = s[i++]))
    { 
        if(currChar != c)
            out[currLen++] = currChar;
    }
    free(s);
    out[currLen] = '\0';
    return out;
}
char* reverse(char* s)
{
    int len = strlen(s);
    char* out = malloc(len + 1);
    int j = 0;
    int i = len - 1;
    for(; i >= 0; --i, ++j)
    {
        out[j] = s[i];
    }
    out[j + 1] = '\0';
    return out;
}
