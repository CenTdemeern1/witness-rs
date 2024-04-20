fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/shaders/");
    // Yes, this is very cursed but unfortunately rust3ds doesn't seem to look in the $OUT_DIR
    let out_dir = std::path::Path::new("romfs/shaders/");
    std::fs::read_dir("src/shaders/").unwrap().for_each(|shader_direntry| {
        let shader_direntry = shader_direntry.unwrap();
        let dest_path = out_dir.join(shader_direntry.file_name());
        assert!(
            std::process::Command::new("picasso")
                .arg("-o")
                .arg(format!("{}.bin", dest_path.display()))
                .arg(shader_direntry.path())
                .status()
                .expect("Couldn't find/run picasso. Do you have devkitPro devkitARM binaries (usually in `/opt/devkitpro/devkitARM/bin`) on your PATH?")
                .success(),
            "Shader assembly was not successful."
        );
    })
}
