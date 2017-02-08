import socket
from random import randint

sock = socket.create_connection(("localhost", 8888))
while True: 
    random_string = str(randint(1, 100000))
    sock.send(random_string.encode("ASCII"))
    ret = sock.recv(len(random_string) + 100).decode("ASCII")
    if not random_string == ret:
        print("there was a difference!; random_string was {}\n and ret was {}\n".format(random_string, ret))
        break
