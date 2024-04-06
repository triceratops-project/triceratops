fn main() {
    tonic_build::configure()
        .compile(
            &[
                "../../proto/triceratops.proto",
                "../../proto/admin.proto",
                "../../proto/user.proto",
            ],
            &["../../proto"],
        )
        .unwrap();
}
