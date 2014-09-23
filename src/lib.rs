use std::io::{Reader, Writer, IoResult};
use std::io::net::tcp::TcpStream;

pub struct Connection {
    sock: TcpStream,
}

impl Connection {
    pub fn new(addr: &str, port: u16) -> IoResult<Connection> {
        let sock = try!(TcpStream::connect(addr, port));

        let mut ret = Connection {
            sock: sock,
        };

        try!(ret.send(b"  V1"));

        Ok(ret)
    }

    fn send(&mut self, data: &[u8]) -> IoResult<()> {
        let w = &mut self.sock as &mut Writer;
        w.write(data.as_slice())
    }
}
