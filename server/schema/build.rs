use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = Path::new("../../proto").canonicalize()?;
    let proto_files = std::fs::read_dir(&proto_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?.path();
            let ext = entry.extension()?.to_str()?;
            (ext == "proto").then_some(entry)
        })
        .collect::<Vec<_>>();

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .build_transport(true)
        .compile_protos(&proto_files, &[proto_dir])?;
    Ok(())
}
