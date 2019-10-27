extern crate ssh2ex;
use std::io::Read;
use std::net::TcpStream;

use ssh2ex::SSH2;

struct SSH_CONNECT_INFO {
    host: String,
    port: u16,
    user: String,
    pass: Option<String>,
    agent_auth: bool,
}

fn callback_ls(mut ch: ssh2::Channel) -> String {
    ch.exec("ls /").unwrap();
    let mut s = String::new();
    ch.read_to_string(&mut s).unwrap();
    s.replace("\n", " ")
}

fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    let socket = TcpStream::connect(("192.168.9.24", 22))?; //(format!("{}:{}", self.host, self.port))?;

    let mut ssh = SSH2::new(socket);

    ssh.connect("wood", Some("jkl"))?;
    println!("ls / -> {}", ssh.sendcmd(callback_ls)?);

    Ok(())
}
