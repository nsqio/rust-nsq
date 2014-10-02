extern crate nsq;

fn main() {
    let mut conn = match nsq::Connection::new("127.0.0.1", 4150) {
        Err(why) => fail!("{}", why),
        Ok(conn) => conn,
    };
    let _ = conn.send(b"SUB test test\n");
    let data = match conn.read_frame() {
        Err(why) => fail!("{}", why),
        Ok(data) => data,
    };
    println!("{}", data);
}
