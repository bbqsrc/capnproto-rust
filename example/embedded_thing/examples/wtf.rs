use capnp::serialize::io::Buffer;

fn main() {
    let mut buf = [0u8; 256];
    let mut buf = Buffer::new(&mut buf[..]);
    let wtf = embedded_thing::foo(&mut buf);
    // println!("{:?}", &buf[..12]);

    let message_reader =
        capnp::serialize::read_message(&mut buf, capnp::message::ReaderOptions::new())
            .expect("read");
    let thing = message_reader
        .get_root::<embedded_thing::bike_capnp::bike::Reader>()
        .expect("wat");
    println!("{:?}", thing.get_id());
    println!("{:?}", thing.get_owner());
}
