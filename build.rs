use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=assets/app-icon.res");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let resource = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest directory"))
            .join("assets/app-icon.res");
        println!(
            "cargo:rustc-link-arg-bin=hangar-flow={}",
            resource.display()
        );
    }
}
