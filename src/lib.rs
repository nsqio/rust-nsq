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

    pub fn send(&mut self, data: &[u8]) -> IoResult<()> {
        let w = &mut self.sock as &mut Writer;
        w.write(data.as_slice())
    }

    pub fn recv(&mut self) -> IoResult<Vec<u8>> {
        let r = &mut self.sock as &mut Reader;
        let size = try!(r.read_be_u32());
        let data = try!(r.read_exact(size as uint));
        Ok(data)
    }
}
