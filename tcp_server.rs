use std::io::*;
use std::net::TcpListener;
use std::{process, thread, time};

fn main() -> std::io::Result<()> {
    // Create a listener
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
    println!("Rust TCP server established listening on address {:?} with process id {:?}", listener.local_addr().unwrap(), process::id());

    // Spawn a thread to handle each incoming requests separately
    for stream in listener.incoming() {
        thread::spawn(|| {
            // Open stream and send connection response
            let mut stream = stream.unwrap();
            let connection_response = format!("A thread with process id {:?} has been assigned to this connection to {:?} from {:?}\r\n",
                thread::current().id(), stream.local_addr().unwrap(), stream.peer_addr().unwrap());
            print!("{}", connection_response);
            stream.write(connection_response.as_bytes()).expect("Error occured when trying to send connection response");

            // Begin infinite loop of sending messages
            let mut message_number: i32 = 0;
            loop {
                // Send mesasge and handle possible errors
                match stream.write(format!("message {}\r\n", message_number).as_bytes()) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Threat {:?} unable to send message to {:?} with error {:?}", thread::current().id(), stream.peer_addr().unwrap(), err);
                        break;  // Break out of the infinite loop.
                        },
                };

                // Increment message number and sleep
                message_number += 1;
                thread::sleep(time::Duration::from_millis(1000));
            }
            println!("Thread {:?} terminating", thread::current().id());
        });
    }
    Ok(())
}