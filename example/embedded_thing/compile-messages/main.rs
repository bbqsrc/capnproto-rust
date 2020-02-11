fn main() {
    let out_dir = std::env::current_dir().expect("have cwd").join("..").join("src");
    std::fs::create_dir_all(&out_dir).expect("mkdir");
    let out_dir = out_dir.canonicalize().expect("canonicalize");

    let bike = std::env::current_dir().expect("have cwd").join("..").join("bike.capnp").canonicalize().expect("canonicalize");

    capnpc::CompilerCommand::new()
        .file(&bike)
        .output_path(&out_dir)
        .run()
        .expect("compiling schema");
}
