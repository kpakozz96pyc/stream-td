use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var("PROFILE").unwrap());
    let assets_src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("assets");
    let assets_dst = out_dir.join("assets");

    if assets_dst.exists() {
        fs::remove_dir_all(&assets_dst).unwrap();
    }
    fs::create_dir_all(&out_dir).unwrap();
    fs::create_dir_all(&assets_dst).unwrap();
    fs_extra::dir::copy(
        &assets_src,
        &out_dir,
        &fs_extra::dir::CopyOptions::new().overwrite(true).copy_inside(true),
    )
        .unwrap();
}