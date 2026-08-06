fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vendored = protoc_bin_vendored::protoc_bin_path()
        .map_err(|e| format!("Failed to get vendored protoc: {}", e))?;
    // SAFETY: This is called in build.rs before any threads are spawned.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("PROTOC", &vendored);
    }

    let proto_root = "../kcm-interface/proto";
    let proto_file = format!("{}/kcm.proto", proto_root);

    println!("cargo:rerun-if-changed={}", proto_file);
    println!("cargo:rerun-if-changed=build.rs");

    tonic_build::configure().compile_protos(&[proto_file], &[proto_root])?;

    Ok(())
}
