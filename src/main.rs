extern crate nix;
use nix::sys::socket::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let sock = socket(AddressFamily::Inet, SockType::Stream, SockFlag::empty(), None)?;
    let ip_addr = IpAddr::new_v4(1, 1, 1, 1);
    let port = 80;
    let sockaddr = SockAddr::new_inet(InetAddr::new(ip_addr, port));
    connect(sock, &sockaddr)?;

    println!("Hello, world!");
    Ok(())
}
