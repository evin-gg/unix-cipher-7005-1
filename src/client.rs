mod networking_util;

#[allow(unused_imports)]
use nix::sys::socket::*;


use networking_util::{
    client_arg_validation, create_socket, format_send, client_check_validpath
};
use::std::{process, env};
use std::os::fd::AsRawFd;

fn main() {

    // get user args
    let args: Vec<String> = env::args().collect();
    match client_arg_validation(args.clone()) {
        Ok(())=> {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    // Check if the path is valid
    match client_check_validpath(&args) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    // make a socket 
    let sock = match create_socket() {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("[CLIENT] Socket Creation Error {}", e);
            process::exit(1);
        }
    };

    // Create an address
    let addr = match UnixAddr::new(args[3].as_str()) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[CLIENT] Address Error {}", e);
            process::exit(1);
        }
    };

    // Connect to the server
    match connect(sock.as_raw_fd(), &addr) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("[CLIENT] Error Connecting to Server {}", e);
            process::exit(1);
        }
    };

    // Send the formatted data
    match format_send(args, &sock) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("[CLIENT] Error Sending Data {}", e);
            process::exit(1);
        }
    };

    // Receive the response
    let mut buffer =[0u8; 1024];
    let received_bytes = match recv(sock.as_raw_fd(), &mut buffer, MsgFlags::empty()) {
        Ok(n) => {println!("[CLIENT] Received {} bytes", n); n},
        Err(e) => {
            eprintln!("[CLIENT] Error Receiving Data {}", e);
            process::exit(1);
        }
    };

    let buffer_slice = &buffer[..received_bytes];
    println!("[CLIENT] Encoded Message: {:?}", str::from_utf8(buffer_slice).unwrap());
}