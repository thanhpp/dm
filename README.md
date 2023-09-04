# dm - daemons manager

Want to run a command without keeping your ssh session?

## Why does this app exist?

Have you ever run a command in a ssh session but then you want to turn off your laptop to go home?
This is my solution to keep those commands running without maintaining a connection to the server.

## How can I do it?

- It would be like a simplified supervisord. 
- There are 2 components
  - A manager to runs command and monitor their status.
  - A command registrator to tell the manager what to run.

