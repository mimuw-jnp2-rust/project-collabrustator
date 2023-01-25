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
- Reqwest (communication between frontend and REST Api)
- Uuid (generating unique ids (for room ids))
- log + env_logger (logging)
- redis (Redis client library)

## Additional tech stack

- Database (for now- Redis)

## Running the application

To run the application locally, you need Redis server up and running (the app stores each room's code piece inside a redis database). To install redis, you can check [this](https://redis.io/docs/getting-started/installation/) link.

The code is divided into two parts- server and client part.

To run the server side, go to the 'backend' directory and run

```
cargo run
```

To check if everyting is working, go to [localhost:8000/health/check](localhost:8000/health/check) (or any url localhost:8000/health/{String}) and check whether it returns 'check' (or the String value provided in the url other than 'check').

To run the client side, go to the 'frontend' directory and run

```
trunk serve
```

Then the website should be available under localhost:8080 (or any other free port; the information should be provided in the terminal after executing the 'serve' command).

If the backend service is exposed under the port 8000, the project parts should communicate properly (that is, the code is saved for each room, it gets automatically updated for all room participants via Websockets + compiling/running it should produce a result).

In other case (the address/port of the backend server is different than expected localhost:8000) the Trunk.toml file (located in the frontend/ directory) should be updated in order to establish communication between the two parts (the first proxy record with address "http://127.0.0.1:8000/" refers to REST Api and the second one- to the Websocket).

It is also possible to host the project and make it public (to use it on multiple devices):

- Thanks to the Trunk.toml file (which creates a Proxy for all the services with urls in the file) we just have to expose the port with the frontend part

- One of the easiest solutions (and one which was used during testing/development) is to use ngrok

- To do so, you can create a free-tier account [here](https://ngrok.com/) and then follow [this](https://ngrok.com/download) 3 step tutorial (install ngrok, add token, expose port (in our case port 8080, unless the frontend part started on another one)))

- After exposing the port from your main device, you should see the url from which you can access the page from all devices (example url: https://8c5e-178-73-34-162.eu.ngrok.io) (it changes every time you expose a port via ngrok)

## Example code snippets:

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
- [Redis library documentation](https://docs.rs/redis/latest/redis/)
- [Websocket server in rust using Warp](https://blog.logrocket.com/how-to-build-a-websocket-server-with-rust/)
- [Adding Websocket connection to yew frontend app](https://blog.devgenius.io/lets-build-a-websockets-project-with-rust-and-yew-0-19-60720367399f)
