import socket


# Questions for review:
# - Which line creates the TCP socket?
# - Which line opens the network connection?
# - Which bytes are the HTTP request line?
# - Which bytes are the request header?
# - Where do the headers end?

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("example.com", 80))
s.sendall(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")

response = s.recv(4096)
print(response.decode())

s.close()
