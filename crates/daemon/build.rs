fn main() {
    tonic_build::configure()
        .compile(
            &[
                "../../proto/triceratops.proto",
                "../../proto/server.proto",
                "../../proto/user.proto",
            ],
            &["../../proto"],
        )
        .unwrap();
}
