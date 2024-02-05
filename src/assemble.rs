use super::config::SysCall;

pub fn assemble(arch: &str, os: &str, regs: Vec<String>, syscalls: Vec<SysCall>, infile: String)-> String {
    let mut output: Vec<String> = Vec::new();
    let func_split = regex::Regex::new(r"[\s\(\),]+").expect("Regex error");

    let mut skip_section = false;

    for line in infile.lines() {
        let line = line.trim();

        if line.starts_with("#") {
            let parts = line.replace("#", "").replace("{", "");
            let mut parts_iter = parts.split(".");
            let sec_arch = parts_iter.next().expect("Section architecture not found").trim();
            let sec_os = parts_iter.next().expect("Section OS not found").trim();
            if sec_arch != arch || sec_os != os {
                skip_section = true;
            }
        }
        else if skip_section == true && line.contains("}") {
            skip_section = false;
            continue;
        }
        else if skip_section == true {
            continue;
        }
        else if line.contains("}") {
            continue;
        }
        else if line.starts_with(">") {
            let mut parts = func_split.split(line);
            let fname = parts.next().expect("Function name not found").replace(">", "");
            let args = parts.collect::<Vec<&str>>();

            let mut inc_reg = 1;
            let mut call_output = String::from("");

            if fname == ":syscall" {
                inc_reg = 0;
            }
            else {
                let syscall = syscalls.iter().find(|s| s.name == fname).expect("Syscall not found");
                call_output.push_str(format!("\tmov\t{}, {}\n", regs[0], syscall.code).as_str());
            }

            for (i, arg) in args.iter().enumerate() {
                if !arg.is_empty() {
                    call_output.push_str(format!("\tmov\t{}, {}\n", regs[i + inc_reg], arg).as_str());
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