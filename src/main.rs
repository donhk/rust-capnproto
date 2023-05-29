use capnp::message::Builder;
use capnp::serialize;

mod schemas;
use schemas::foo;

use capnp::message::ReaderOptions;

fn main() {
    let mut message_builder = Builder::new_default();
    let mut foo_object = message_builder.init_root::<foo::Builder>();
    foo_object.set_name("John");
    foo_object.set_id(23);
    let data = serialize::write_message_to_words(&message_builder);
    println!("foo {:?} sent over the wire 🔌🔌🔌🔌🔌🔌🔌🔌", data);

    let reader = serialize::read_message(
        data.as_slice(),
        ReaderOptions::new(),
    ).unwrap();

    let person = reader.get_root::<foo::Reader>().unwrap();
    let name = person.get_name().unwrap();
    let id = person.get_id();
    println!("Name: {name}");
    println!("Name: {id}");
}
