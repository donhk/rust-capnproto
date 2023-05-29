//  capnp compile -orust:$OUT_DIR --src-prefix=schema schema/foo.capnp schema/bar.capnp
extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .output_path("src/schemas")
        .src_prefix("schema/")
        .file("schema/foo.capnp")
        .file("schema/bar.capnp")
        .run().unwrap();
}