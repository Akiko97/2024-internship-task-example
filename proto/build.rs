use std::process::Command;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
    env,
};

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
                .type_attribute(".", "#[derive(proto_gen::CmdID)]")
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

    implement_cmd_id(Path::new("out/msg.rs")).unwrap();
}

fn implement_cmd_id(path: &Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    let mut cmd_id_attr = None;
    for line in reader.lines() {
        let line = line?;
        if line.contains("CmdID: ") {
            cmd_id_attr = Some(make_cmd_id_attr(&line).unwrap());
        } else {
            output.push(line);
            if let Some(attr) = cmd_id_attr.take() {
                output.push(attr);
            }
        }
    }

    fs::write(path, output.join("\n").as_bytes())?;
    Ok(())
}

fn make_cmd_id_attr(line: &str) -> Option<String> {
    let cmd_id = line.split("CmdID: ").nth(1)?.parse::<u16>().ok()?;
    Some(format!("#[cmdid({cmd_id})]"))
}
