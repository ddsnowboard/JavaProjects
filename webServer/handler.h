enum serverStatus {CONTINUE, STOP, PRINT};
struct inputStruct {
    int fd;
    // This is something that the function can set so that 
    // the caller will shut down based on the right input from the 
    // client
    enum serverStatus *channel;
};
void *handle(void* input);
