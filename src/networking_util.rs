#![allow(dead_code)]

pub mod cipher;

use std::os::fd::OwnedFd;
use nix::errno::Errno;
use nix::sys::socket::{
    socket, AddressFamily, SockType, SockFlag
};
use nix::sys::socket::*;
use std::os::fd::AsRawFd;
use nix::unistd::close;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use nix::sys::signal;
use nix::sys::signal::SigAction;
use std::path::Path;

use cipher::split_payload;


pub static CATCH_SIGINT: AtomicBool = AtomicBool::new(false);

extern "C" fn sigint_handler(_sig: i32) {
    CATCH_SIGINT.store(true, Ordering::SeqCst);
}

pub fn sigint_init() -> SigAction {
    return signal::SigAction::new(
        signal::SigHandler::Handler(sigint_handler),
        signal::SaFlags::empty(),
        signal::SigSet::empty());
}

// Client Setup functions
pub fn client_check_validpath(argpath: &Vec<String>) -> Result<(), String> {
    let path = Path::new(&argpath[3]);
    if path.exists() {
        Ok(())
    } else {
        Err("[CLIENT] Socket does not exist".to_string())
    }
}

pub fn client_arg_validation(args: Vec<String>) -> Result<(), String> {
    if args.len() != 4 || args[2].parse::<i32>().is_err() {
        return Err("[CLIENT] Usage: <message> <shift> <socketpath>".to_string());
    }
    else {
        Ok(()) 
    }
}

pub fn create_socket() -> Result<OwnedFd, Errno> {
    return socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), None);

    
}

pub fn format_send(args: Vec<String>, sock: &OwnedFd) -> Result<(), Errno> {
    let payload = format!("{} {}", args[2], args[1]);

    match send(sock.as_raw_fd(), payload.as_bytes(), MsgFlags::empty()) {
        Ok(_bytes) => {return Ok(())},
        Err(e) => {
            return Err(e);
        }
    };
}

// Server Setup functions

pub fn server_loop(sock: &OwnedFd) {
    let clientfd = match accept(sock.as_raw_fd()) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("[SERVER] {}", e);
            return;
        }
    };

    let mut buf = [0u8; 1024];
    let read_bytes = match recv(clientfd, &mut buf, MsgFlags::empty()){
        Ok(n) => {println!("[SERVER] Received {} bytes", n); n},
        Err(e) => {println!("[SERVER] Error: {}", e); 0},
    };

    let encoded_message = split_payload(&buf[..read_bytes]);
    match send(clientfd, encoded_message.as_bytes(), MsgFlags::empty()) {
        Ok(n) => println!("[SERVER] Sent {} bytes", n),
        Err(e) => println!("[SERVER] Error Sending Data {}", e),
    };

    match close(clientfd) {
        Ok(()) => {},
        Err(e) => {
            println!("[SERVER] Error Closing Client File Descriptor {}", e);
        }
    }
}

pub fn server_check_validpath(argpath: &Vec<String>) -> Result<(), String> {

    let path = Path::new(&argpath[1]);
    if path.exists() {
        Err("[SERVER] Old socket found, it will be unlinked and replaced".to_string())
    } else {
        Ok(())
    }
}

pub fn server_arg_validation(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err("[SERVER] Usage: <socketpath>".to_string());
    }
    else {
        Ok(()) 
    }
}
