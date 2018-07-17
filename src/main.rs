extern crate nix;
use nix::sys::socket::*;
use std::io;
use std::io::Write;
use std::result::Result;

fn main() -> Result<(), Box<std::error::Error>> {
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None)?;
    let ip_addr = IpAddr::new_v4(94, 142, 241, 111);
    let port = 23;
    let sockaddr = SockAddr::new_inet(InetAddr::new(ip_addr, port));
    connect(sock, &sockaddr)?;

    loop {
        let mut buf = [0u8; 1024];
        let len = recv(sock, &mut buf, MsgFlags::empty())?;
        let new_bytes = &buf[..len];

        io::stdout().write(new_bytes);
    }
    // println!("Hello, world!");
    Ok(())
}
