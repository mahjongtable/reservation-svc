fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile_protos(&["protos/reservation/reservation.proto"], &["protos"])?;

    println!("cargo:run-if-changed=protos/reservation/reservation.proto");

    Ok(())
}
