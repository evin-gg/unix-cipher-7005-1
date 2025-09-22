#![allow(unused_imports)]
mod networking_util;


use nix::sys::socket::*;
use networking_util::{
    create_socket, server_loop, sigint_init, CATCH_SIGINT, server_check_validpath, server_arg_validation
};

use::std::{process, env};
use::std::os::fd::AsRawFd;
use nix::unistd::*;
use std::sync::atomic::Ordering;
use nix::sys::signal;


fn main() {

    let sig_action = sigint_init();
    unsafe { match signal::sigaction(signal::SIGINT, &sig_action) {
        Ok(_sigaction) => {},
        Err(e) => {
            eprintln!("[SERVER] Error setting up Signal Handler {}", e);
            process::exit(1);
        }
    } };

    // Get args and verify
    let user_args: Vec<String> = env::args().collect();
    match server_arg_validation(user_args.clone()) {
        Ok(())=> {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }

    // Check if the path is valid
    match server_check_validpath(&user_args) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    // declare path and unlink to ensure
    let path = &user_args[1];
    
    match unlink(path.as_str()) {
        Ok(()) => {},
        Err(_e) => {}
    };

    // make a socket 
    let sock = match create_socket() {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("[SERVER] Error: {}", e);
            process::exit(1);
        }
    };

    // Create an address 
    let addr = match UnixAddr::new(path.as_str()) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[SERVER] Address Error {}", e);
            process::exit(1);
        }
    };

    // bind socket to the address 
    match bind(sock.as_raw_fd(), &addr) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("[SERVER] Bind Error {}", e);
            process::exit(1);
        }
    };

    // listen for connections 
    match listen(&sock, Backlog::new(5).expect("Invalid backlog size")) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("[SERVER] Listening Error {}", e);
        }
    };

    println!("[SERVER] Listening for connections");
    while !CATCH_SIGINT.load(Ordering::SeqCst) {
        server_loop(&sock);
    }


    println!("[SERVER] Unlinking File");
    match unlink(path.as_str()) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("[SERVER] Error unlinking path {}", e);
        }
    };
}
