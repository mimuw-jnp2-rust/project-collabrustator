# CollabRustAtor

## Authors

- Jan Wojtach (@YannVoytaa on GitHub)

## Description

CollabRustAtor is going to be a live collaborative rust code editor and compiler (web app, client-server architecture)

## Features

- user can create a 'room' available for others,
- single room contains one text document editable by all room members simultaneously (potential feature- more advanced file structure: more files/directories)
- apart from editing the document, users can compile/run it as a rust file (potential feature- allowing for choosing programming languages other than rust) (for security reasons (infinite loops/...), there would be a time limit for a program to run)

## Plan

In the first part I'm going to implement basic web-based rust code editor: no rooms, no concurrent editing, only editing and compiling/running would be available.

In the second part I'm going to add:

1. rooms- files can be saved to database (one file per one room), other users can see the changes after refreshing the page (or by clicking the button to reload the file content only- for better user experience);
2. concurrent editing- every client will be connected to WebSocket server and send/receive pieces of information about new file changes in certain room.

## Libraries

- Yew (client-side/frontend)
- Warp (server-side/backend)
- Serde (serialization)
- Syntect (syntax highlighting)

## Additional tech stack

- Docker
- Database (ie. MySQL)

## Running the application

To run the application locally, you need 'docker' up and running (the app executes every code piece inside a docker container for the security reasons). To install docker, you can check [this](https://docs.docker.com/get-docker/) link.

The code is divided into two parts- server and client part.

To run the server side, go to the 'backend' directory and run

```
cargo run
```

To check if everyting is working, go to [localhost:8000/room](localhost:8000/room) and check whether it returns 'ok' message.

To run the client side, go to the 'frontend' directory and run

```
trunk serve
```

Then the website should be available under localhost:8080 (or any other free port; the information should be provided in the terminal after executing the 'serve' command).

Example code snippets:

```
fn main() {
    println!("abc");
}
// shows 'abc' in the terminal as a result;
```

```
fn main() {
    println("abc");
}
// gives an error with a standard explanation of a rust compiler;
```

```
fn main() {
    let a = 1;
    println!("abc");
}
// gives a warning, but shows 'abc' as a result as well;
```

```
fn main() {
    loop {
    }
}
// results in timeout after 10 seconds.
```

```
fn main() {
    let mut x = 1;
    loop {
        x -= 1;
        x /= x;
    }
}
// results in a runtime error and 'panic' message.
```

## Useful resources

These are the links to the websites that helped me build the current project and learn about the technologies used:

- [Yew tutorial](https://yew.rs/docs/tutorial)
- [Building an API with Rust using Tokio and Warp](https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a)
- [Syntect docs and examples](https://github.com/trishume/syntect)
