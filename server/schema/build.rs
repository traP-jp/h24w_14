use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = Path::new("../../proto").canonicalize()?;
    let proto_files = std::fs::read_dir(&proto_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("proto"))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .compile_protos(&proto_files, &[proto_dir])?;
    Ok(())
}
