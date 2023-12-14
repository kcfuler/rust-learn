fn main() {
    let mut config = prost_build::Config::new();
    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("scr/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
