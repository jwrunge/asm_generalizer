use super::config::SysCall;

pub fn assemble(regs: Vec<String>, syscalls: Vec<SysCall>, infile: String)-> String {
    let mut output: Vec<String> = Vec::new();
    let func_split = regex::Regex::new(r"[\s\(\),]+").expect("Regex error");

    for line in infile.lines() {
        let line = line.trim();
        if line.starts_with(">") {
            let mut parts = func_split.split(line);
            let fname = parts.next().expect("Function name not found").replace(">", "");
            let args = parts.collect::<Vec<&str>>();

            println!("Outputting syscall: {}", fname);

            let syscall = syscalls.iter().find(|s| s.name == fname).expect("Syscall not found");
            let mut call_output = String::from("");

            call_output.push_str(format!("\tmov\t{}, {}\n", regs[0], syscall.code).as_str());
            for (i, arg) in args.iter().enumerate() {
                if !arg.is_empty() {
                    call_output.push_str(format!("\tmov\t{}, {}\n", regs[i + 1], arg).as_str());
                }
            }
            call_output.push_str(format!("\tint\t0x80\n").as_str());
            output.push(call_output);
        }
        else {
            output.push(line.to_string());
        }
    }

    output.join("\n")
}