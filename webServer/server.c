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

#define BUF_SIZE 20000

void* echoSocket(void* arg);
void handleBindError();
void handleReadError();

int main(int argc, char** argv) {
    int sockFD = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in source;
    source.sin_family = AF_INET;
    source.sin_port = htons(8888);
    source.sin_addr.s_addr = INADDR_ANY;
    int bindResult = bind(sockFD, (struct sockaddr *) &source, sizeof(source));
    if(bindResult < 0)
        handleBindError();

    fd_set readFds;
    FD_SET(sockFD, &readFds);
    listen(sockFD, 5);
    pthread_t burnerThread;
    for(;;) {
        long clientFD = (long) accept(sockFD, NULL, NULL);
        pthread_create(&burnerThread, NULL, echoSocket, (void*) clientFD);
    }
    shutdown(sockFD, SHUT_RDWR);
    close(sockFD);
    return 0;
}

void* echoSocket(void* arg) {
    int sock = (int) ((long) arg);
    printf("Received connection %d\n", sock);
    char recvBuffer[BUF_SIZE];
    ssize_t bytesRead = 0;;
    do {
        bytesRead = read(sock, recvBuffer, BUF_SIZE);
        if(bytesRead < 0)
        {
            handleReadError();
            break;
        }
        else
        {
            write(sock, recvBuffer, bytesRead);
        }
    } while(bytesRead != 0);
    close(sock);
    printf("Lost connection %d\n", sock);
    return NULL;
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
