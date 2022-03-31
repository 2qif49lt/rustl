fn main() {
    prost_build::Config::new()
        .out_dir("src/protocol/pb")
        .compile_protos(&["msg.proto"], &["src/protocol/pb"])
        .unwrap();
}