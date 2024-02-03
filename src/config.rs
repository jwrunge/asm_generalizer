use std::collections::HashMap;
use regex::Regex;

pub struct ConstArgs {
    reg: String,
    val: i32,
}

pub struct AutoArgs {
    reg: String,
    func: String,
    reg_arg: String,
}

pub struct AsmMap {
    const_args: Option<Vec<ConstArgs>>,
    user_arg_regs: Option<Vec<String>>,
    auto_args: Option<AutoArgs>,
}

fn load_asm_map_def_from_module(arch: &str, os: &str)-> String {
    let exe_dir = std::env::current_dir().expect("Error: cannot get current exe directory");
    let mod_path = std::path::Path::join(
        &exe_dir,
        format!("modules/{}/{}.calls", arch, os),
    );
    println!("{}", mod_path.display());
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

    println!("FUNC: {}", func_name);

    let match_const_args = Regex::new(r"").unwrap();
    let match_user_args = Regex::new(r"").unwrap();
    let match_auto_args = Regex::new(r"").unwrap();

    let const_args = match_const_args.find_iter(line);
    for arg in const_args {
        println!("CONST: {}", arg.as_str());
    }

    Err("Error")
}

pub fn get_asm_maps(arch: &str, os: &str) -> HashMap<String, AsmMap> {
    let asm_map_def = load_asm_map_def_from_module(arch, os);
    
    let mut asm_map: HashMap<String, AsmMap> = HashMap::new();
    println!("{}", asm_map_def);
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