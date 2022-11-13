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
