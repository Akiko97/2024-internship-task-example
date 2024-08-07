use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

pub fn main() {
    let proto_files = [
        "msg.proto",
    ];
    for proto_file in proto_files {
        let proto_file_path = Path::new(proto_file);
        if proto_file_path.exists() {
            println!("cargo:rerun-if-changed={proto_file}");
            prost_build::Config::new()
                .out_dir("out")
                .compile_protos(&[proto_file], &["."])
                .unwrap();
        }
        let proto_file_name = proto_file_path
            .file_stem().unwrap().to_str().unwrap();
        let project_path = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut yarn_build = Command::new("npx")
            .arg("pbjs")
            .arg("--ts")
            .arg(format!("{project_path}/{proto_file_name}_pb.ts"))
            .arg(format!("{project_path}/{proto_file}"))
            .spawn()
            .expect("Could not open yarn to compile proto file");
        yarn_build.wait().expect("Could not compile proto file to JavaScript file");
        if Path::new(format!("{project_path}/{proto_file_name}_pb.ts").as_str()).exists() {
            fs::rename(
                format!("{project_path}/{proto_file_name}_pb.ts"),
                format!("{project_path}/../web/src/proto/{proto_file_name}_pb.ts")
            ).expect("Could not move TypeScript proto file");
        }
    }
}
