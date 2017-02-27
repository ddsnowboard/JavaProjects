#include "handler.h"
#include <string.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

#define BUFFER_SIZE 2000

void chomp(char* s) {
    while(*s != '\n')
        s++;
    *s = '\0';
}

void *handle(void* input) {
    struct inputStruct info = *(struct inputStruct*) input;
    free(input);
    int fd = info.fd;
    ssize_t bytesRead;
    char buf[BUFFER_SIZE];
    while(*info.errorHolder != STOP && (bytesRead = read(fd, buf, BUFFER_SIZE)) != 0) {
        chomp(buf);
        if(strcmp(buf, "quit") == 0) {
            *info.errorHolder = STOP;
            break;
        }
    }
    close(fd);
}
