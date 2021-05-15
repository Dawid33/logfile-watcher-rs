# logfile-watcher-rs

 [![Build Status](https://github.com/Dawid33/logfile-watcher-rs/workflows/CI/badge.svg)](https://github.com/Dawid33/logfile-watcher-rs/actions?query=workflow%3ACI)
<img src="./assets/v0.0.1.gif" alt="Demo of logfile-watcher gif">

`logfile-watcher-rs` is a client + server application that allows real time viewing of remote files over a websocket. Both the client and server use a documented json for configuration.

## TODO :
### General
Rework the event loop mechanism

Websocket api

Serverside stuff

### Windows compatibility
write crossterm implementation of events_termion in events_crossbeam, or better yet just merge them into one.
write crossterm backend in main()
## Wishlist:
