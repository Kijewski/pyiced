use std::fs::OpenOptions;
use std::io::Read;

fn main() {
    pyo3_build_config::add_extension_module_link_args();

    let mut f = OpenOptions::new().read(true).open("setup.cfg").unwrap();
    let mut version = String::new();
    f.read_to_string(&mut version).unwrap();
    let version = version
        .lines()
        .find_map(|line| {
            const PREFIX: &str = "version = ";
            if !line.starts_with(PREFIX) {
                return None;
            }
            Some(line[PREFIX.len()..].trim())
        })
        .unwrap();
    println!("cargo:rerun-if-changed=setup.cfg");
    println!("cargo:rustc-env=pyiced-version={}", version);
}
