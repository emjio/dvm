use crate::utils::{deno_bin_path, dvm_root};
use crate::version::current_version;
use anyhow::Result;
use semver_parser::version::parse as semver_parse;
use std::fs;
use std::process::exit;

pub fn exec(version: Option<String>) -> Result<()> {
  let target_version = match version {
    Some(target_version) => match semver_parse(&target_version) {
      Ok(ver) => ver,
      Err(_) => {
        eprintln!("Invalid semver");
        exit(1)
      }
    },
    None => unimplemented!(),
  };
  let target_exe_path = deno_bin_path(&target_version);

  if !target_exe_path.exists() {
    eprintln!("deno v{} is not installed.", target_version);
    exit(1)
  }

  let current_version = current_version().unwrap();

  if current_version == target_version.to_string() {
    println!("Failed: deno v{} is in use.", target_version);
    exit(1);
  }

  let dvm_dir = dvm_root().join(format!("{}", target_version));

  fs::remove_dir_all(&dvm_dir).unwrap();
  println!("deno v{} removed.", target_version);

  Ok(())
}
