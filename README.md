# MyTeams

Distributed collaborative messaging platform in C with binary TCP protocol.

## Overview

Client-server architecture supporting teams, channels, threads, private direct messages, and disk persistence using non-blocking I/O multiplexing (`select`).

## Getting Started

```bash
# Compile server and client
make

# Launch server: ./myteams_server <port>
./myteams_server 4242

# Launch client: ./myteams_cli <ip> <port>
./myteams_cli 127.0.0.1 4242
```
