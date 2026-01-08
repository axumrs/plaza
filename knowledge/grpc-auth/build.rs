fn main() -> anyhow::Result<()> {
    tonic_prost_build::configure()
        .out_dir("src/pb")
        .compile_protos(&["protos/hello.proto"], &["protos"])?;
    Ok(())
}
