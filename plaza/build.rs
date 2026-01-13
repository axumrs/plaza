fn main() -> anyhow::Result<()> {
    let mut proto_files = vec![];
    for entry in std::fs::read_dir("protos")? {
        let entry = entry?;
        let path = entry.path();
        let md = entry.metadata()?;
        if md.is_file() && path.extension().unwrap_or_default() == "proto" {
            proto_files.push(path.as_path().to_string_lossy().to_string());
        }
    }

    tonic_prost_build::configure()
        .out_dir("src/pb")
        .compile_protos(
            proto_files
                .iter()
                .map(|p| p.as_str())
                .collect::<Vec<_>>()
                .as_slice(),
            &["protos"],
        )?;
    Ok(())
}
