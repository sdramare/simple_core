use std::process::{self, Command};

fn main() {
    let mut qemu = Command::new("qemu-system-x86_64");
    #[cfg(debug_assertions)]
    {
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
