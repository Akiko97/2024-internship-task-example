use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;

pub fn main() {
    let project_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_path = format!("{project_path}/..");
    let web_path = format!("{workspace_path}/web");
    println!("cargo:rerun-if-changed={workspace_path}/server/server.json");
    println!("cargo:rerun-if-changed={web_path}");

    let mut yarn = Command::new("yarn")
        .current_dir(web_path.clone())
        .spawn()
        .expect(format!("Could not run `yarn` in {web_path}").as_str());
    yarn.wait().expect("Error in running `yarn`");

    if Path::new(format!("{web_path}/server.json").as_str()).exists() {
        fs::remove_file(format!("{web_path}/server.json"))
            .expect(format!("Could not delete {web_path}/server.json").as_str());
    }
    fs::copy(
        format!("{workspace_path}/server/server.json"),
        format!("{web_path}/server.json")
    ).unwrap();

    let mut yarn = Command::new("yarn")
        .arg("build")
        .current_dir(web_path.clone())
        .spawn()
        .expect(format!("Could not run `yarn build` in {web_path}").as_str());
    yarn.wait().expect("Error in running `yarn build`");

    if Path::new(format!("{web_path}/dist").as_str()).exists() {
        if Path::new(format!("{workspace_path}/dist").as_str()).exists() {
            fs::remove_dir_all(format!("{workspace_path}/dist"))
                .expect(format!("Could not delete {workspace_path}/dist").as_str());
        }
        fs::rename(
            format!("{web_path}/dist"),
            format!("{workspace_path}/dist")
        ).unwrap();
    }
}
