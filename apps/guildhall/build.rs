//! Windows only: embed the app icon and version info (VERSIONINFO) into `guildhall.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `GUILDHALL_REQUIRE_WINRES=1` (set
//! by the release packaging) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/guildhall.ico");
    println!("cargo:rerun-if-env-changed=GUILDHALL_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/guildhall.ico")
        .set("ProductName", "Guildhall")
        .set("FileDescription", "Guildhall")
        .set("CompanyName", "Guildhall contributors")
        .set("LegalCopyright", "Copyright (c) the ArtCraft Launcher and Guildhall authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "guildhall.exe")
        .set("InternalName", "guildhall");
    if let Err(e) = res.compile() {
        if std::env::var_os("GUILDHALL_REQUIRE_WINRES").is_some() {
            println!("cargo:warning=embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=guildhall.exe built without icon/version resources: {e}");
    }
}
