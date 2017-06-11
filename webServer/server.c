#include <stdlib.h>
#include <unistd.h>
#include <sys/time.h>
#include <sys/types.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <sys/select.h>
#include <netinet/in.h>
#include <stdio.h>
#include <errno.h>

#include <pthread.h>
#include "handler.h"

#define DEFAULT_PORT 8888

struct socketAcceptorInfo {
    int fd;
    enum serverStatus *chan;
};

void* acceptSockets(void* inVal);
void* echoSocket(void* arg);
void handleBindError();
void handleReadError();

int main(int argc, char** argv) {
    int portNo;
    if(argc == 2)
        portNo = atoi(argv[1]);
    else
        portNo = DEFAULT_PORT;


    int sockFD = socket(AF_INET, SOCK_STREAM, 0);
    // I don't understand any of this stuff
    struct sockaddr_in source;
    source.sin_family = AF_INET;
    source.sin_port = htons(portNo);
    source.sin_addr.s_addr = INADDR_ANY;

    int bindResult = bind(sockFD, (struct sockaddr *) &source, sizeof(source));
    if(bindResult < 0)
        handleBindError();

    fd_set readFds;
    FD_SET(sockFD, &readFds);
    listen(sockFD, 5);
    enum serverStatus channel = CONTINUE;

    // We don't really care about what the thread does, 
    // so we don't have to hang on to it.
    pthread_t _acceptorThread;
    struct socketAcceptorInfo acceptorInformation;
    acceptorInformation.fd = sockFD;
    acceptorInformation.chan = &channel;
    if(pthread_create(&_acceptorThread, NULL, acceptSockets, (void*) &acceptorInformation) != 0) {
        printf("There was a problem creating the listener thread");
        exit(1);
    }

    while(channel != STOP) {
        // block
    }

    shutdown(sockFD, SHUT_RDWR);
    close(sockFD);

    pthread_exit(NULL);
    return 0;
}

void* acceptSockets(void* inVal) {
    struct socketAcceptorInfo info = *(struct socketAcceptorInfo*) inVal;
    int sockFD = info.fd;
    enum serverStatus *channel = info.chan;
    while(*channel != STOP) {
        int clientFD = accept(sockFD, NULL, NULL);
        struct inputStruct *input = malloc(sizeof(struct inputStruct));
        input->fd = clientFD;
        input->channel = channel;
        pthread_t _threadId;
        pthread_create(&_threadId, NULL, handle, (void*) input);
    }
}

void handleReadError() {
    printf("Error with read: ");
    switch(errno) {
        case EAGAIN:
            printf("Error EAGAIN\n");
            break;
        case EBADF:
            printf("Error EBADF\n");
            break;
        case EFAULT:
            printf("Error EFAULT\n");
            break;
        case EINTR:
            printf("Error EINTR\n");
            break;
        case EINVAL:
            printf("Error EINVAL\n");
            break;
        case EIO:
            printf("Error EIO\n");
            break;
        case EISDIR:
            printf("Error EISDIR\n");
            break;
        default:
            printf("Unknown error %d\n", errno);
            break;
    }
}

void handleBindError() {
    printf("Error with bind: ");
    switch(errno) {
        case EACCES:
            printf("Error was EACCES\n");
            break;

        case EADDRINUSE:
            printf("Error was EADDRINUSE\n");
            break;

        case EBADF:
            printf("Error was EBADF\n");
            break;

        case EINVAL:
            printf("Error was EINVAL\n");
            break;

        case ENOTSOCK:
            printf("Error was ENOTSOCK\n");
            break;

        case EADDRNOTAVAIL:
            printf("Error was EADDRNOTAVAIL\n");
            break;

        case EFAULT:
            printf("Error was EFAULT\n");
            break;

        case ELOOP:
            printf("Error was ELOOP\n");
            break;

        case ENAMETOOLONG:
            printf("Error was ENAMETOOLONG\n");
            break;

        case ENOENT:
            printf("Error was ENOENT\n");
            break;

        case ENOMEM:
            printf("Error was ENOMEM\n");
            break;

        case ENOTDIR:
            printf("Error was ENOTDIR\n");
            break;

        case EROFS:
            printf("Error was EROFS\n");
            break;

        default:
            printf("Unknown error on bind: %d\n", errno);
            break;
    }
    exit(1);
}
