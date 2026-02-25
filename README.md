# mini-http-serve

i built this because i was tired of using abstractions i didn’t fully understand

frameworks are cool.  
but at some point i wanted to see what actually happens

so this project is a minimal HTTP server written in pure Rust, using only the standard library

---

## purpose

this was built to:

- understand how HTTP works at a lower level
- practice systems programming in Rust
- deal with raw TCP streams
- manually parse requests and construct responses
- get comfortable handling concurrency without magic

basically: remove the comfort layer and see what’s really happening

---

## how it works

at a high level:

1. binds to a TCP port
2. listens for incoming connections
3. spawns a thread per connection
4. reads raw bytes from the TCP stream
5. parses the HTTP request line and headers
6. builds an HTTP response manually
7. writes the response back to the stream

everything is done explicitly

---

## what it supports

- basic `GET` requests
- simple routing
- static responses (for now)
- minimal error handling

---

## what i learned

- HTTP is simple until it isn’t
- parsing text protocols is more fragile than it looks
- Rust makes you think about ownership even when you're just handling sockets
- concurrency is easy to start and harder to scale properly

---

## future improvements

- thread pool instead of spawning unlimited threads
- better request parsing
- proper HTTP status handling
- maybe async version just for comparison

---

this is not meant to compete with real frameworks.
