FROM rust
WORKDIR /usr/src/app
COPY . .
EXPOSE 3000
RUN rustc tcp_server.rs
CMD ["./tcp_server"]