
use micropb_gen::{ Generator };

// Generate Rust module from .proto files
fn proto_generate() {
    let mut gen = Generator::new(); 
    gen.use_container_heapless()
        .configure(".Send.payload", micropb_gen::Config::new().max_bytes(125))
        .configure(".Received.payload", micropb_gen::Config::new().max_bytes(125))
        .add_protoc_arg("-Iproto")
        .compile_protos(
            &[
                "ieee_802_15_4.proto",
            ],
            std::env::var("OUT_DIR").unwrap() + "/proto.rs",
        )
        .unwrap();
    println!("cargo:rerun-if-changed=proto");
}

fn main() {
    proto_generate();
}