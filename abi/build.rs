fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/pb")
        .compile_protos(&["protos/reservation/reservation.proto"], &["protos"])
        .unwrap();

    println!("cargo:run-if-changed=protos/reservation/reservation.proto");
}
