use clap::{load_yaml, App};

mod types;

mod file;
use file::*;

mod lexer;
use lexer::{
    lex_instr,
    lex_direct,
    lex_func
};

mod analyzer;
use analyzer::{
    analyze_instr,
    //analyze_direct,
    analyze_func
};

mod compile;
use compile::into_nasm;

fn main() {
    // Allocating memory for files' names
    let mut is_filename: String = String::new();
    let out_filename: String;

    let mut is_functions = false;
    let mut filenames_func: Vec<types::Indent> = Vec::new();
    let mut functions: Vec<types::Function> = Vec::new();

    let yaml = load_yaml!("cli.yaml");
    let cli = App::from_yaml(yaml).get_matches();

    // COMPILE
    if let Some(matches) = cli.value_of("input") {
        is_filename = matches.to_string();
    };

    if !check_ext(is_filename.clone(), 0) { std::process::exit(1) }

    out_filename = is_filename.replace(".gis", ".asm");

    let code = load_file(is_filename);

    let directives = lex_direct(code.clone());
    for directive in directives {
        match directive {
            types::Directive::Use(mut files_i) => {
                is_functions = true;
                filenames_func.append(&mut files_i);
            }
        }
    }

    let instructions = lex_instr(code);

    
    if is_functions {
        for fs in filenames_func {
            let fs_filename = fs.0.clone();
            if !check_ext(fs_filename.clone(), 1) { std::process::exit(1) }
            let functions_code = load_file(fs_filename);
            functions.append(&mut lex_func(functions_code));
        }
        if !analyze_func(&functions) { 
            eprintln!("\nFAILED TO CHECK FUNCTIONS");
            std::process::exit(1);
        }
    }


    let (ok, variables) = analyze_instr(&instructions);
    if !ok {
        eprintln!("\nFAILED TO CHECK");
        std::process::exit(1);
    }

    let nasm = into_nasm(instructions, variables);

    store_file(nasm, out_filename);


    // TODO
    // if let Some(directives) = directives_o {
    //     for directive in directives {
    //         println!("{:?}", directive);
    //         // match directive {
    //         //     Directive::Use(filnames) => 
    //         // }
    //     }
    // }
}

fn check_ext(filename: String, type_ext: u8) -> bool {
    let smt: Vec<&str> = filename.split('.').collect();
    match type_ext {
        0 => {
            if smt[smt.len()-1] != "gis" {
                eprintln!("Instruction set should be with extension '.gis'");
                std::process::exit(1);
            } else { return true }
        },
        1 => {
            if smt[smt.len()-1] != "gfs" {
                eprintln!("Functions set should be with extension '.gfs'");
                std::process::exit(1);
            } else { return true }
        },
        _ => unreachable!()
    }
}

