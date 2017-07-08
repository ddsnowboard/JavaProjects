#include <errno.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#define MAX_WORD_LEN 100
#define MAX_WORD_COUNT 1000
#define WORDLIST "/usr/share/dict/words"
#define FILENAME "cipher.txt"
#define KEY_LENGTH 3
#define ERROR 1
#define OK 0

int letterCount(FILE*);
void decrypt(char key[], int code[], char out[], int n);
int iterateKey(char key[]);
void stringify(char* destString, char sourceChars[], int n);
int fgetpair(FILE* f);

int main(int argc, char** argv)
{
    FILE *in = fopen(FILENAME, "r");
    int length = letterCount(in);
    int* letters = malloc(sizeof(int) * length);
    int* walker = letters;
    int currentNumber;
    while((currentNumber = fgetpair(in)) >= 0)
        *walker++ = currentNumber;
    fclose(in);

    char key[KEY_LENGTH] = {'a', 'a', 'a'};
    char* out = calloc(length + 1, sizeof(char));
    do {
        decrypt(key, letters, out, length);
        // printf("%s\n\n", out);
    }
    while(iterateKey(key) == OK);
    free(out);
    free(letters);
    return 0;
}

// Returns the amount of numbers that are to be converted to an ASCII character
// in f. 
int letterCount(FILE *f)
{
    int originalOffset = ftell(f);
    fseek(f, 0, SEEK_SET);
    char curr;
    int count = 0;
    while((curr = getc(f)) >= 0)
    {
        if(curr == ',')
        {
            count++;
        }
    }
    fseek(f, originalOffset, SEEK_SET);
    return count + 1;
}

int iterateKey(char key[]) {
    // Assert key is valid
    for(int i = 0; i < KEY_LENGTH; i++) {
        if(key[i] > 'z' || key[i] < 'a') {
            char st[KEY_LENGTH + 1];
            st[KEY_LENGTH] = '\0';
            strncpy(st, key, KEY_LENGTH);

            printf("Something bad happened; the key is not right; key is %s", st);
        }
    }

    for(int i = KEY_LENGTH - 1; i >= 0; i--) {
        if(key[i] == 'z') {
            key[i] = 'a';
            continue;
        }
        else {
            key[i]++;
            return OK;
        }
    }
    printf("You ran out of keys!");
    return ERROR;
}

void stringify(char* destString, char sourceChars[], int n) {
    for(int i = 0; i < n; i++)
        destString[i] = sourceChars[i];
    destString[n] = '\0';
}

int fgetpair(FILE* f) {
    char pair[KEY_LENGTH + 1];
    for(int i = 0; i < KEY_LENGTH + 1; i++)
        pair[i] = '\0';
    char curr = getc(f);
    if(curr == EOF)
        return -1;
    int idx = 0;
    do {
        pair[idx++] = curr;
    } while((curr = getc(f)) >= 0 && curr != ',');
    return atoi(pair);
}

void decrypt(char key[], int code[], char out[], int n) {
    for(int i = 0; i < n; i++) {
        out[i] = code[i] ^ key[i % KEY_LENGTH];
    }
}

char** wordsplit(char* inString) {
    char buffer[MAX_WORD_LEN];
    char* strings[MAX_WORD_COUNT];
    int countOfStrings = 0;
    int idx = 0;
    while(*inString != '\0') {
        if(*inString == ' ') {
            if(idx != 0) {
                buffer[idx] = '\0';
                char* newString = malloc(idx + 1);
                strcpy(newString, buffer);
                strings[countOfStrings++] = newString;
            }
            idx = 0;
            continue;
        }
        else {
            buffer[idx++] = *inString;
        }
    }
    char** out = malloc(countOfStrings);
    for(int i = 0; i < countOfStrings; i++)
        out[i] = strings[i];
    return out;
}
