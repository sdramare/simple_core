use std::{
    env,
    process::{self, Command},
};

fn main() {
    let mut qemu = Command::new("qemu-system-x86_64");
    let is_debug = env::var("DEBUG").map(|val| val == "1").unwrap_or_default();
    if is_debug {
        qemu.arg("-gdb");
        qemu.arg("tcp::8864");
        qemu.arg("-S");
    }

    qemu.arg("-serial");
    qemu.arg("stdio");
    qemu.arg("-drive");
    qemu.arg(format!("format=raw,file={}", env!("BIOS_IMAGE")));
    let exit_status = qemu.status().unwrap();
    process::exit(exit_status.code().unwrap_or(-1));
}
