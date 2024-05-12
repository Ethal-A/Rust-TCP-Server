# Rust TCP Server
A basic rust TCP server for learning purposes.

This server operates using TCP on port 3000. Each connection made to port 3000 is assigned a thread which will send messages messages back to the client for as long as the connection is maintained.

A Dockerfile has been created to make the execution and use of this program easier.

## How to Run Using Docker
These instructions assume you are running docker in Ubuntu and that you are using a terminal within your local repository of Rust TCP Server. Arguments to the instructions are tailored to what I believe is most convenient.
1. Begin by [installing docker](https://docs.docker.com/engine/install/ubuntu/) if you have not already.
2. Build the image from the Dockerfile inside this repository using the command `sudo docker build -t tcp-rust-server .`. This will assign the image a tag of `tcp-rust-server` which will make it easier to find later.
3. Run the image using the command `sudo docker run --name rust-tcp-server --rm tcp-rust-server`. This will assign the container a name of `rust-tcp-server` which will make it easier to kill later if need be and the argument `--rm tcp-rust-server` will delete the container if you exit for your convenience.

## How to Find the Docker Container IP Address
To find the docker container IP address:
1. Find the ID of the `rust-tcp-server` container using the command `sudo docker container ps` (assuming that it is currently running).
2. Find the IP address of the container using the command `sudo docker inspect <id> | grep "IPAddress"`. Note that you only need to provide the first few characters in the id.

## How to Connect to Docker Container Using Netcat (nc)
This is just one simple way to connect to the Docker container and observe the program working.
1. Open up a new terminal whilst you are currently running the `rust-tcp-server` docker container.
2. In the new terminal, use the command  `nc <Docker Container IP Address> 3000`.

## How to Kill the Docker Container
If you are having trouble stopping the docker container, you may need to kill it.
1. Open up a new terminal.
2. Use the command `sudo docker container kill rust-tcp-server` in the new terminal.