#![allow(non_snake_case)]


use crate::instr::*;


pub fn lex_code(source_code: Vec<u8>) -> (Vec<Instruction>, Option<Vec<Directive>>) {
    let instructions = lex_instr(source_code);
    for instruction in &instructions.0 {
        println!("{:?}", instruction);
    }
    for directive in &instructions.1 {
        println!("{:?}", directive);
    }
    instructions
}

fn lex_instr(source_code: Vec<u8>) -> (Vec<Instruction>, Option<Vec<Directive>>) {
    let mut used_chars: [char; 76] = [0 as char; 76];
    let spec_chars = [':', '#', '[', ']', '!', '\n', '*', '&', '+', '-', '<', '>', '@', '|'];
    for (i,c) in ('a'..='z').enumerate() { used_chars[i] = c; }
    for (i,c) in ('A'..='Z').enumerate() { used_chars[i+26] = c; }
    for (i,c) in ('0'..='9').enumerate() { used_chars[i+52] = c; }
    for (i,c) in spec_chars.iter().enumerate() { used_chars[i+62] = *c; }
  
    

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut directives: Vec<Directive> = Vec::new();
    let mut variables: Vec<Variable> = Vec::new();

    let mut comment = false;

    let mut isDirective = false;
    let mut pushDirective = false;
    let mut parseDirective = false;
    let mut DirStr = String::new();
    let mut parseDirArgs = false;
    let mut DirArgStr = String::new();
    let mut pushDirArg = false;
    let mut DirArgs: Vec<String> = Vec::new();

    let mut isVariable = false;
    let mut parseSizeVar = false;
    let mut pushSizeVar = false;
    let mut parseIndentVar = false;
    let mut SizeVarStr = String::new();
    let mut SizeVar = Size::Byte;
    let mut VarName = String::new();
    let mut indent = String::new();
    let mut parseValueVar = false;
    let mut pushVar = false;
    let mut pushValVar = false;
    let mut ValVar = Value::Byte(0);
    let mut ValVarStr = String::new();
    let mut parseValFun = false;
    let mut parseValueFun = false;

    let mut column: usize = 0;
    let mut row: usize = 1;

    for sym_code in source_code {
        column += 1;
        if sym_code == 0xA {
            if column == 1 { 
                column = 0;
                row += 1;
                continue
            }
            column = 0;
            row += 1;
        }

        let symbol: char = sym_code as char;
        
        if symbol == ' ' { continue }

        if symbol == ';' {
            comment = true;
            continue
        }
        if comment {
            if sym_code == 0xA {
                comment = false;
                continue
            } else {
                continue
            }
        }


        if !used_chars.contains(&symbol) {
            error(0, row, column, symbol)
        }

        if isDirective {
            if !parseDirective {
                match symbol {
                    '\n' => {
                        isDirective = false;
                        pushDirective = true;
                    },
                    'A'..='Z' => parseDirective = true,
                    _ => unreachable!()
                }
            } 
            if parseDirective {
                if !parseDirArgs {
                    match symbol {
                        '<' => parseDirArgs = true,
                        'A'..='Z' => DirStr.push(symbol),
                        _ => unreachable!()
                    }
                } else {
                    if symbol == '>' {
                        parseDirArgs = false;
                        isDirective = false;
                        parseDirective = false;
                        pushDirArg = true;
                        pushDirective = true;
                    } else if symbol == '|' {
                        pushDirArg = true;
                    } else {
                        DirArgStr.push(symbol);
                    }

                    if pushDirArg {
                        pushDirArg = false;
                        DirArgs.push(DirArgStr);
                        DirArgStr = String::new();
                    }
                }
            }

            if pushDirective {
                pushDirective = false;
                match get_type_dir(DirStr.clone()) {
                    Ok(t) => {
                        if t {
                            match get_directive_args(DirStr, DirArgs) {
                                Ok(dir) => directives.push(dir),
                                Err(_) => error(6, row, column, symbol)
                            }
                        } else {
                            unreachable!();
                            //match get_directive(DirStr) {
                            //    Ok(dir) => directives.push(dir),
                            //    Err(_) => unreachable!()
                            //}
                        }
                    },
                    Err(_) => error(5, row, column, symbol)
                }
                DirStr = String::new();
                DirArgs = Vec::new();
            }
        } else {
            if !isVariable {
                if symbol == '\n' { continue }
                match symbol {
                    '!' => isDirective = true,
                    'B'|'W' => isVariable = true,
                    _ => unreachable!(symbol)
                }
            }
        }

        if isVariable {
            if !parseSizeVar && !parseIndentVar && !parseValueVar {
                match symbol {
                    'B'|'W'|'D' => parseSizeVar = true,
                    'a'..='z'|'0'..='9' => parseIndentVar = true,
                    '#' => parseValueVar = true,
                    '@' => parseValueFun = true,
                    _ => unreachable!(symbol)
                }
            }

            if parseSizeVar {
                match symbol {
                    'a'..='z'|'0'..='9' => {
                        parseSizeVar = false;
                        pushSizeVar = true;
                        parseIndentVar = true;
                    },
                    'A'..='Z' => (),
                    _ => unreachable!()
                }
                if !pushSizeVar {
                    SizeVarStr.push(symbol)
                } else {
                    pushSizeVar = false;
                    match get_size(SizeVarStr) {
                        Ok(sz) => SizeVar = sz,
                        Err(_) => error(2, row, column, symbol)
                    }
                    SizeVarStr = String::new();
                }
            }

            if parseIndentVar {
                if symbol == ':' {
                    parseIndentVar = false;
                    VarName = indent;
                    indent = String::new();
                } else {
                    indent.push(symbol);
                }
            }

            if parseValueVar {
                match symbol {
                    '#' => continue,
                    '0'..='9' => (),
                    '\n' => {
                        parseValueVar = false;
                        pushVar = true;
                        pushValVar = true;
                    },
                    _ => unreachable!(symbol)
                }
                if pushValVar {
                    pushValVar = false;
                    ValVar = match SizeVar {
                        Size::Byte => {
                            match ValVarStr.parse::<u8>() {
                                Ok(val) => Value::Byte(val),
                                Err(_) => { error(3, row, column, symbol); Value::Byte(0) }
                            }
                        },
                        Size::Word => {
                            match ValVarStr.parse::<u16>() {
                                Ok(val) => Value::Word(val),
                                Err(_) => { error(3, row, column, symbol); Value::Word(0) }
                            }
                        }
                    };
                    ValVarStr = String::new();
                } else {
                    ValVarStr.push(symbol);
                }
            }

            // if parseValueFun {
            //     match symbol {
            //         '@' => (),
            //         'a'..='z'|'0'..='9' => parseValFuncName = true,
            //         '[' => {
            //             parseValFuncArgs = true;
            //             parseValFuncName = false;
            //         },
            //         ']' => {
            //             parseValueFun = false;
            //             parseValFuncArgs = false;
            //             pushFunc = true;
            //         },
            //         _ => unreachable!()
            //     }

            //     if parseValFuncName {

            // }


            if pushVar {
                pushVar = false;
                isVariable = false;
                let var = Variable {
                    name: Indent(VarName),
                    size: SizeVar,
                    value: ValVar,
                };
                println!("{:?}", var);
                variables.push(var);

                VarName = String::new();
                parseSizeVar = false;
                parseIndentVar = false;
                parseValueVar = false;
            }
        }
    }
    if directives.len() > 0 {
        (instructions, Some(directives))
    } else {
        (instructions, None) 
    }
}



/*
 * error codes:
 *  - 0: unknown token
 *  - 1: unspecifed function signature
 *  - 2: unknown variable size
 *  - 3: failed to parse immediate value
 *  - 4: incorrect function
 *  - 5: Unknown directive
 *  - 6: Failed to parse arguments of directive
 */
fn error(err_code: u8, row: usize, column: usize, symbol: char) {
    println!("{}", symbol as u8);
    match err_code {
        0 => eprintln!("Unknown token at {}:{}", row, column),
        1 => eprintln!("Unspecifed function signature at {}:{}", row, column),
        2 => eprintln!("Unknown variable size at {}:{}", row, column),
        3 => eprintln!("Failed to parse immediate value at {}:{}", row, column),
        4 => eprintln!("Incorrect function ar {}:{}", row, column),
        5 => eprintln!("Unknown directive at {}:{}", row, column),
        6 => eprintln!("Failed to parse arguments of directive at {}:{}", row, column),
        _ => panic!("Unreachable error code")
    }
    std::process::exit(1);
}

fn get_size(size_str: String) -> Result<Size, ()> {
    match size_str.as_str() {
        "BYTE" => Ok(Size::Byte),
        "WORD" => Ok(Size::Word),
        _ => Err(())
    }
}

fn get_type_dir(dir: String) -> Result<bool, ()> {
    match dir.as_str() {
        "USES" => Ok(true),
        _ => Err(())
    }
}

fn get_directive_args(dir: String, args: Vec<String>) -> Result<Directive, ()> {
    let arguments: Vec<Indent> = args.iter().map(| arg: &String | Indent((*arg).clone()) ).collect();
    match dir.as_str() {
        "USES" => Ok(Directive::Use(arguments)),
        _ => Err(())
    }
}

