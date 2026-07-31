fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "../../crates/kcm-interface/proto";
    tonic_build::configure()
        .compile_protos(&[format!("{}/kcm.proto", proto_root)], &[proto_root])?;
    Ok(())
}
