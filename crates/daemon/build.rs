fn main() {
    tonic_build::configure()
        .compile(&["../../proto/triceratops.proto"], &["../../proto"])
        .unwrap();
}
