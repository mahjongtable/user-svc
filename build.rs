fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::Config::g::compile_protos("protos/mahjongapis/user/user.proto")?;
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["protos/user/user.proto"], &["protos"])?;
    Ok(())
}
