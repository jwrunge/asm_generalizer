mod config;
mod assemble;

fn main() {
    let mut arch = String::from("");
    let mut os = String::from("");
    let mut infile = String::from("");
    let mut outfile = String::from("");

    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        if arg.starts_with("--arch=") {
            arch = arg.replace("--arch=", "").trim().to_string();
        }
        else if arg.starts_with("--os=") {
            os = arg.replace("--os=", "").trim().to_string();
        }
        else if arg.starts_with("-o=") {
            outfile = arg.replace("-o=", "").trim().to_string();
        }
        else if !arg.starts_with("--") && arg != &args[0] {
            infile = arg.to_string();
        }
    }

    if arch == "" || os == "" || infile == "" {
        println!("Error: missing required arguments");
        println!("Usage: {} --arch=<arch> --os=<os> <infile> [-o=<outfile>]", args[0]);
        return;
    }

    if outfile == "" {
        let infile_no_ext = infile.split(".").next().unwrap();
        outfile = format!("{}.asm", infile_no_ext);
    }

    let filedata = std::fs::read_to_string(&infile)
        .expect(&format!("Error: cannot read file {}", infile));

    let (registers, syscalls) = config::get_arch_os_defs(&arch, &os);
    let output = assemble::assemble(&arch, &os, registers, syscalls, filedata);

    match std::fs::write(&outfile, output) {
        Ok(_) => println!("Success: output written to {}", outfile),
        Err(e) => println!("Error: cannot write to file {}: {}", outfile, e),
    }

}
