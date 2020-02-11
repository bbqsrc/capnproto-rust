#![no_std]

pub mod bike_capnp;
use capnp::serialize::io::Buffer;

pub fn foo(mut buf: &mut Buffer) {
    let mut message = capnp::message::Builder::new_default();
    let mut bike = message.init_root::<bike_capnp::bike::Builder>();
    bike.set_id(667);
    bike.set_owner("Joe");
    // bike.set_owner("KatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarinaKatarina");
    let msg_size = bike.total_size().expect("know total size");

    capnp::serialize::write_message(&mut buf, &message).expect("didnt explode");
}
