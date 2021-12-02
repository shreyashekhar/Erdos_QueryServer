# What we learned

Overall rust syntax and understanding:
- Overall syntax
- Closures
- Enums
- Generic Functions
- Borrowing vs Moving

Useful libraries:
- Serde
- Warp
- Tokio

# What we completed

- ability to:
  - create socket-based endpoints with warp
  - recieve JSON requests and provide JSON responses

- abstractions to:
  - pass a web socket and a handler function and automatically 
    set up all of the piping to send requests and response to/from 
    the function
  - parse a JSON string to a struct with handling of incorrect 
    formatting

# What we attempted (but didn't complete)

- asbtracting a bit more of the socket mapping logic into a function
  - (some typing issue)
- forwarding another stream to the socket
  - (issue with borrowing)

# What's next

- fix the above issues
- flesh out the endpoints (need to define specific requirements)
- integrate with ERDOS
- built more frontend