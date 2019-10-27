extern crate snafu;
extern crate ssh2;

use snafu::{ResultExt, Snafu};
use ssh2::{Channel, Session};
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("io: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("ssh2: {}", source))]
    SSHErr { source: ssh2::Error },
}
type Result<T, E = Error> = std::result::Result<T, E>;

// #[derive(Debug)]
// pub enum SSH2Error {
//     Io(::std::io::Error),
//     Protocol(::ssh2::Error),
// }
// impl From<::std::io::Error> for SSH2Error {
//     fn from(err: ::std::io::Error) -> SSH2Error {
//         SSH2Error::Io(err)
//     }
// }
// impl From<::ssh2::Error> for SSH2Error {
//     fn from(err: ::ssh2::Error) -> SSH2Error {
//         SSH2Error::Protocol(err)
//     }
// }

pub struct SSH2 {
    session: Session,
    // host: String,
    // port: u16,
}

impl<'ch> SSH2 {
    pub fn new(socket: TcpStream) -> Self {
        let mut session = Session::new().unwrap(); //ok_or(::std::io::Error::new(  ::std::io::ErrorKind::ConnectionAborted,  "session-object not generated."))?;
        session.set_tcp_stream(socket);
        let ssh = SSH2 {
            //socket: None,
            session,
            // host: host.to_owned(),
            // port: port,
        };
        ssh
    }

    fn is_authenticated(&self) -> bool {
        self.session.authenticated()
        // if ! {
        //     Err(::std::io::Error::new(
        //         ::std::io::ErrorKind::PermissionDenied,
        //         "Authentication failure.",
        //     ))
        // } else {
        //     Ok(true)
        // }
    }

    fn authentication(&self, user: &str, pass: Option<&str>) -> bool {
        let sess = &self.session;
        if let Some(pass) = pass {
            // password
            sess.userauth_password(user, pass);
        } else {
            // ssh-agent
            println!("ssh-agent");
        }
        self.is_authenticated()
    }

    pub fn connect(
        &mut self,
        //socket: TcpStream,
        user: &str,
        pass: Option<&str>,
    ) -> Result<()> {
        //let mut session = Session::new().unwrap(); //ok_or(::std::io::Error::new(  ::std::io::ErrorKind::ConnectionAborted,  "session-object not generated."))?;
        //session.set_tcp_stream(socket);
        //session.userauth_password(username: &str, password: &str);
        //session.channel_direct_tcpip(self.host.as_str(), self.port, None)?;
        self.session.handshake().context(SSHErr {})?;
        //self.socket = Some(socket);
        //self.session = Some(session);
        self.authentication(user, pass);
        Ok(())
    }

    pub fn sendcmd(&self, callback: fn(Channel) -> String) -> Result<String> {
        let mut channel = self.session.channel_session().context(SSHErr {})?;
        Ok(callback(channel))
    }
}
