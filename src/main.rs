use std::{env::args, io::Write, path::PathBuf, process::Command};

use tempfile::NamedTempFile;

fn main() {
    let path = PathBuf::from(
        args()
            .nth(1)
            .expect("expected path to efi file as an argument"),
    )
    .canonicalize()
    .expect("failed to canonicalize executable path");
    let ovmf = include_bytes!("OVMF.fd");
    let mut ovmf_file = NamedTempFile::new().expect("failed to create temporary ovmf file");

    ovmf_file
        .write_all(ovmf)
        .expect("failed to write to temporary ovmf file");

    Command::new(env!("CARGO_BIN_FILE_UEFI_RUN"))
        .args([
            "-b",
            ovmf_file
                .path()
                .to_str()
                .expect("failed to turn temporary ovmf file path into `&str`"),
        ])
        .arg(path)
        .status()
        .expect("failed to execute uefi-run");
}
