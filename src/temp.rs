fn parse_syscalls(arch: &str, os: &str)-> String {
    let exe_dir = std::env::current_dir().expect("Error: cannot get current exe directory");
    let mod_path = std::path::Path::join(
        &exe_dir,
        format!("modules/{}/{}.cmap", arch, os),
    );
    let contents = std::fs::read_to_string(mod_path)
        .expect(&format!("Error: cannot read module {}/{}", arch, os));
    contents
}

fn load_asm_map_def_from_module(arch: &str, os: &str)-> String {
    let exe_dir = std::env::current_dir().expect("Error: cannot get current exe directory");
    let mod_path = std::path::Path::join(
        &exe_dir,
        format!("modules/{}/{}.cmap", arch, os),
    );
    if mod_path.exists() {
        let contents = std::fs::read_to_string(mod_path)
            .expect(&format!("Error: cannot read module {}/{}", arch, os));
        contents
    } else {
        panic!("Error: cannot find module {}.{}. Is it installed?", arch, os);
    }
}

fn convert_line_to_map(line: &str) -> Result<(String, AsmMap), &str> {
    let func_regex = Regex::new(r"^\w+\s").unwrap();
    let func_name = match func_regex.find(line) {
        Some(capture) => capture.as_str(),
        None => return Err("Error: cannot find function name"),
    };

    let mut asm_map = AsmMap {
        const_args: vec![],
        user_arg_regs: vec![],
        auto_args: vec![],
    };

    let match_const_args = Regex::new(r"\S+?:\S+?").unwrap();
    let match_var_args = Regex::new(r"\[.*?\]").unwrap();
    let split_auto_args = Regex::new(r"[|()]").unwrap();

    let const_args = match_const_args.find_iter(line);
    for arg in const_args {
        let mut parts = arg.as_str().split(":");
        let reg = match parts.next() {
            Some(reg) => reg,
            None => return Err("Error: cannot find register name for constant argument"),
        };
        let val = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => return Err("Error: cannot find value for constant argument"),
        };

        asm_map.const_args.push(ConstArgs {
            reg: reg.to_string(),
            val: val,
        });
    }

    let var_args = match_var_args.find_iter(line);
    for arg in var_args {
        let arg = arg.as_str();
        if arg.starts_with("[args: ") {
            let regs = arg.replace("[args: ", "").replace("]", "");
            for reg in regs.split_whitespace() {
                asm_map.user_arg_regs.push(reg.to_string());
            }
        }
        else if arg.starts_with("[auto: ") {
            let defs = arg.replace("[auto: ", "").replace("]", "");
            for def in defs.split_whitespace() {
                let mut parts = split_auto_args.split(def);
                let reg = match parts.next() {
                    Some(reg) => reg,
                    None => return Err("Error: cannot find register name for auto argument"),
                };
                let func = match parts.next() {
                    Some(func) => func,
                    None => return Err("Error: cannot find function name for auto argument"),
                };
                let reg_arg = match parts.next() {
                    Some(reg_arg) => reg_arg,
                    None => return Err("Error: cannot find register argument for auto argument"),
                };
                asm_map.auto_args.push(AutoArgs {
                    reg: reg.to_string(),
                    func: func.to_string(),
                    reg_arg: reg_arg.to_string(),
                });
            }
        }
        else {
            return Err("Error: unknown variable argument type")
        }
    }

    Ok((func_name.to_string(), asm_map))
}

pub fn get_asm_maps(arch: &str, os: &str) -> HashMap<String, AsmMap> {
    let asm_map_def = load_asm_map_def_from_module(arch, os);
    
    let mut asm_map: HashMap<String, AsmMap> = HashMap::new();
    for (idx, line) in asm_map_def.lines().enumerate() {
        let m = convert_line_to_map(line);
        match m {
            Ok(m) => {
                asm_map.insert(m.0, m.1);
            }
            Err(e) => {
                eprintln!("Error in module map {}.{} (LINE {}): {}", arch, os, idx + 1, e);
                continue;
            }
        }
    }

    asm_map
}