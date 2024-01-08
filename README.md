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

## Register a linux service

> https://wiki.debian.org/systemd/Services

- Create your service's unit file with the ".service" suffix in the /etc/systemd/system directory
- type: Simple - running in the foreground
- Restart=always
- Environment
  - WorkingDirectory
  - RootDirectory
  - User
  - Group
- dependencies
  - After= (services that must be started before ours) 
- Example
  - ```
    # Contents of /etc/systemd/system/myservice.service
    [Unit]
    Description=My Service
    After=network.target

    [Service]
    Type=simple
    Restart=always
    ExecStart=/usr/local/bin/myservice

    [Install]
    WantedBy=multi-user.target
    ```

## Multiple executable

> https://stackoverflow.com/questions/36604010/how-can-i-build-multiple-binaries-with-cargo

- using [[bin]]

## Refs

- https://doc.rust-lang.org/std/process/struct.Command.html
- https://docs.rs/tokio/latest/tokio/process/struct.Command.html
- https://www.shubhamdipt.com/blog/how-to-create-a-systemd-service-in-linux/