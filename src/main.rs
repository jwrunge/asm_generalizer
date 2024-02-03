mod config;

fn main() {
    let mut arch = String::from("");
    let mut os = String::from("");

    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        if arg.starts_with("--arch=") {
            arch = arg.replace("--arch=", "").trim().to_string();
        }
        else if arg.starts_with("--os=") {
            os = arg.replace("--os=", "").trim().to_string();
        }
    }

    if arch == "" || os == "" {
        println!("Usage: {} --arch=<arch> --os=<os>", args[0]);
        return;
    }

    let asm_maps = config::get_asm_maps(&arch, &os);
}
