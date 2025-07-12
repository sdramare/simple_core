use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
    println!("kernel path: {kernel_path}");
    let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path));
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("../../..");
    //copy the kernel binary to the output directory
    let kernel_bin = PathBuf::from(env::var("CARGO_BIN_FILE_KERNEL").unwrap());
    let kernel_dest = out_dir.join("kernel.bin");
    std::fs::copy(&kernel_bin, &kernel_dest).expect("Failed to copy kernel binary");

    // specify output paths

    let uefi_path = out_dir.join("blog_os-uefi.img");
    let bios_path = out_dir.join("blog_os-bios.img");

    // create the disk images
    disk_builder.create_uefi_image(&uefi_path).unwrap();
    disk_builder.create_bios_image(&bios_path).unwrap();

    // print the paths to the images
    println!("UEFI image created at: {}", uefi_path.display());
    println!("BIOS image created at: {}", bios_path.display());

    println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}
