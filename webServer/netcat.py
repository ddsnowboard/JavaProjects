import socket
from sys import argv

def netcat(host, port, content):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((host, int(port))) 
    s.sendall(content.encode()) 
    s.shutdown(socket.SHUT_WR) 
    while True: 
        data = s.recv(4096) 
        if not data: 
            break 
        print(repr(data)) 
    s.close()

while True:
    netcat("localhost", argv[1], raw_input())
