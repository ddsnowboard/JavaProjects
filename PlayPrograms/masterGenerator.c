#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define MAX_LINE_SIZE 100
// This is just for testing
#define OUTPUT_FILE "MasterCueList123.txt"
#define LS_COMMAND "ls -v1"
#define LS_FILE ".output.txt"

char* getLine(FILE *f);
char* chompExt(char* filename);

int main(int argc, char** argv)
{
    if(!(access(OUTPUT_FILE, F_OK) == -1 || (argc >= 2 && strcmp(argv[1], "-f") == 0)))
    {
        printf("Use -f to force a new file\n");
        return 1;
    }
    // I don't know how to get sh to ignore empty globs, so I'll just worry 
    // about it later and do this for now.
    const char *AUDIO_FORMATS[] = {".wav", ".mp3"};
    const int AUDIO_FORMATS_LEN = 2;
    char *command = (char *) malloc(strlen(LS_COMMAND) + (9 * AUDIO_FORMATS_LEN) + 35);
    strcat(command, LS_COMMAND);
    int i = 0;
    int filesExist = 0;
    for(; i < AUDIO_FORMATS_LEN; ++i)
    {
        char *testCmd = (char *) malloc(15);
        strcat(testCmd, "ls *");
        strcat(testCmd, AUDIO_FORMATS[i]);
        strcat(testCmd, " 2>/dev/null > /dev/null");
        if(system(testCmd) == 0)
        {
            strcat(command, " *");
            strcat(command, AUDIO_FORMATS[i]);
            filesExist = 1;
        }
    }
    if(filesExist != 1)
    {
        printf("There are no music files!\n");
        free(command);
        return 2;
    }
    strcat(command, " > ");
    strcat(command, LS_FILE);
    system(command);
    FILE *lsOutput = fopen(LS_FILE, "r");
    if(lsOutput == NULL)
    {
        printf("Something bad happened!\n");
        fclose(lsOutput);
        remove(LS_FILE);
        return 1;
    }
    // I think I can run through the lines and print them now. So that's nice.
    // I'm going to have to replicate my fancy logic for subjugation or whatever
    // though. Or not...
    char *output;
    FILE *outFile = fopen(OUTPUT_FILE, "w");
    while(strcmp(output = getLine(lsOutput), "") != 0)
    {
        fputs(chompExt(output), outFile);
        fputs(":\n", outFile);
    }
    fclose(outFile);
    fclose(lsOutput);
    remove(LS_FILE);
    return 0;
}

char* getLine(FILE *f)
{
    int allocatedMemory = 32;
    char *almostOut = (char *) malloc(allocatedMemory);
    char *nextBite = (char *) malloc(2);
    do
    {
        if(fread(nextBite, 1, 1, f) != 0)
            strcat(almostOut, nextBite);
        else
            break;
    } while(nextBite[0] != '\n');
    // Sans newline, plus null terminator
    size_t len = strlen(almostOut);
    if(len == 0)
        return "";
    char *out = (char *) calloc(1, len - 1 + 1);
    strncpy(out, almostOut, len - 1);
    free(almostOut);
    return out;
}

char* chompExt(char* filename)
{
    int i = strlen(filename) - 1;
    for(; i > 0; --i)
    {
        if(filename[i] == '.')
        {
            char* out = (char *) malloc(i + 1);
            strncpy(out, filename, i);
            return out;
        }
    }
    return filename;
}
