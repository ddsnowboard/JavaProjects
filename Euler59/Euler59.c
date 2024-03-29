#include <errno.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "hashset.h"

#define MAX_WORD_COUNT 1000
// Found through experimentation
#define LONGEST_WORD_IN_DICTIONARY 26
#define MAX_WORD_LEN LONGEST_WORD_IN_DICTIONARY
#define WORDLIST "/usr/share/dict/words"
#define FILENAME "cipher.txt"
#define KEY_LENGTH 3
#define ERROR 1
#define OK 0
#define WORDS_TO_TEST 20
#define HASHSET_SIZE 5000

char** wordsplit(char* inString, int* numWordsOut);
int letterCount(FILE*);
void decrypt(char key[], int code[], char out[], int n);
int iterateKey(char key[]);
void stringify(char* destString, char sourceChars[], int n);
int fgetpair(FILE* f);
void freeWordlist(char** wordlist, int length);
int testWords(char** wordList, int listLength);
int sumCodes(char* s);
struct HashSet* getDictionary();

static struct HashSet* words = NULL;

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

    char bestKey[KEY_LENGTH] = {'\0', '\0', '\0'};
    int mostWords = 0;

    do {
        decrypt(key, letters, out, length);
        int numWords;
        char** words = wordsplit(out, &numWords);
        // If there is nothing there, we don't have to check, 
        // but we still have to free
        if(words != NULL) {
            int wordsFound = testWords(words, numWords);
            if(wordsFound > mostWords) {
                mostWords = wordsFound;
                memcpy(bestKey, key, sizeof(key));
            }
            printf("There were %d words found with key %c%c%c\n", wordsFound , key[0], key[1], key[2]);
        }
        freeWordlist(words, numWords);
    }
    while(iterateKey(key) == OK);

    if(bestKey[0] == '\0') {
        printf("We couldn't find anything!\n");
    } else {
        decrypt(bestKey, letters, out, length);
        printf("%s\n", out);
        printf("The sum is %d\n", sumCodes(out));
    }

    free(out);
    free(letters);
    hs_free(words);
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
    printf("You ran out of keys!\n");
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

char** wordsplit(char* inString, int* numWordsOut) {
    char buffer[MAX_WORD_LEN];
    char* strings[MAX_WORD_COUNT];
    int countOfStrings = 0;
    int idx = 0;
    while(1) {
        char curr = *(inString++);
        if(curr == ' ' || curr == '\0') {
            if(idx != 0) {
                buffer[idx] = '\0';
                char* newString = malloc(idx + 1);
                strcpy(newString, buffer);
                strings[countOfStrings++] = newString;
            }
            idx = 0;
            if(curr == '\0')
                break;
            else
                continue;
        }
        else {
            if(idx < MAX_WORD_LEN)
                buffer[idx++] = curr;
            else {
                for(int i = 0; i < countOfStrings; i++)
                    free(strings[i]);
                *numWordsOut = 0;
                return NULL;
            }
        }
    }
    char** out = malloc(sizeof(char*) * countOfStrings);
    for(int i = 0; i < countOfStrings; i++)
        out[i] = strings[i];
    *numWordsOut = countOfStrings;
    return out;
}

void freeWordlist(char** wordlist, int length) {
    for(int i = 0; i < length; i++)
        free(wordlist[i]);
    free(wordlist);
}

void chomp(char* s) {
    while(*s) {
        if(*s == ' ' || *s == '\n') {
            *s = '\0';
            return;
        }
        s++;
    }
}

int testWords(char** wordList, int listLength) {
    struct HashSet* words = getDictionary();
    int foundWords = 0;
    for(int i = 0; i < WORDS_TO_TEST && i < listLength; i++) {
        char* currentWord = wordList[i];
        if(hs_contains(words, currentWord)) {
            foundWords++;
        }
    }
    return foundWords;
}

int sumCodes(char* s) {
    int out = 0;
    while(*s)
        out += *s++;
    return out;
}

struct HashSet* getDictionary() {
    if(words == NULL) {
        FILE* wordsFile = fopen(WORDLIST, "r");
        words = hs_create(HASHSET_SIZE);
        char currentDictionaryWord[LONGEST_WORD_IN_DICTIONARY];
        while(fgets(currentDictionaryWord, LONGEST_WORD_IN_DICTIONARY, wordsFile) != NULL) {
            chomp(currentDictionaryWord);
            hs_insert(words, currentDictionaryWord);
        }
        fclose(wordsFile);
    }
    return words;
}
