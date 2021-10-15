use clap::{load_yaml, App};

mod types;

mod file;
use file::*;

mod lexer;
use lexer::lex_instr;

mod analyzer;
use analyzer::analyze_instr;

mod compile;
use compile::into_nasm;

fn main() {
    // Allocating memory for files' names
    let mut is_filename: String = String::new();
    
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from_yaml(yaml).get_matches();

    // COMPILE
    if let Some(matches) = cli.value_of("input") {
        is_filename = matches.to_string();
    }

    let smt: Vec<&str> = is_filename.split('.').collect();
    if smt[smt.len()-1] != "gis" {
        eprintln!("Functions set should be with extension '.gis'");
        std::process::exit(1);
    }


    let code = load_file(is_filename);
    let (instructions, _directives_o) = lex_instr(code);
    // println!("Instructions:");
    // for instruction in &instructions {
    //     println!("{:?}", instruction);
    // }

    let (ok, variables) = analyze_instr(&instructions);
    if ok {
        println!("\nCHECK COMPLETE");
    } else {
        eprintln!("\nFAILED TO CHECK");
        std::process::exit(1);
    }

    let _nasm = into_nasm(instructions, variables);


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
