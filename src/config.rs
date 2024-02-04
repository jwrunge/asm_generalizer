#[derive(Debug)]
pub struct SysCall {
    pub name: String,
    pub code: String,
    pub args: Vec<String>,
}

fn get_config_files(arch: &str, os: &str) -> (String, String) {
    let exe_dir = std::env::current_dir().expect("Error: cannot get current exe directory");
    
    let registers_path = std::path::Path::join(
        &exe_dir,
        format!("modules/{}/registers.csv", arch),
    );
    let syscalls_path = std::path::Path::join(
        &exe_dir,
        format!("modules/os/{}/syscalls.csv", os),
    );

    let registers = std::fs::read_to_string(&registers_path)
        .expect(&format!("Error: cannot read registers definition from {}", registers_path.to_str().unwrap()));
    let syscalls = std::fs::read_to_string(&syscalls_path)
        .expect(&format!("Error: cannot read syscalls definition from {}", syscalls_path.to_str().unwrap()));

    (registers, syscalls)
}

pub fn get_arch_os_defs(arch: &str, os: &str) -> (Vec<String>, Vec<SysCall>) {
    let (registers, syscalls_list) = get_config_files(arch, os);

    let regs_strs: std::str::Split<'_, &str> = registers.split(",");
    let regs: Vec<String> = regs_strs.map(|s| s.to_string()).collect();

    let mut lines = syscalls_list.lines();
    let headers: Vec<&str> = lines.next().expect("Error: syscalls file is empty").split(",").collect();

    let mut syscalls: Vec<SysCall> = Vec::new();
    for line in lines {
        let details = line.split(",");
        let mut name = String::from("");
        let mut args: Vec<String> = Vec::new();
        let mut code = String::from("");

        for (index, val) in details.enumerate() {
            if index == 0 {
                name = val.to_string();
                continue;
            }

            let header = headers.get(index).unwrap_or(&"").to_string();
            if header == arch {
                code = val.to_string();
            }
            else if header.contains("arg") {
                args.push(val.to_string());
            }
        }

        syscalls.push(SysCall {
            name: name,
            code: code,
            args: args,
        });
    }

    (regs, syscalls)
}
