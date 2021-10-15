use crate::types::*;


pub fn into_nasm(instructions: Vec<Instruction>, variables: Vec<Variable>) -> String {
    let mut code = String::new();

    let (u_vars, i_vars) = get_un_init_vars(&variables);
    let instructions = get_executable_instr(instructions);
    // section .bss
    code.push_str("SECTION .bss\n");
    for u_var in u_vars {
        let (size, quantity) = match u_var.size {
            Size::Byte => ("resb", 1_u8),
            Size::Word => ("resw", 1_u8)
        };
        code.push_str(format!("\t{}_:\t{} {}\n", u_var.name.0, size, quantity).as_str());
    }
    code.push('\n');

    // section .data
    code.push_str("SECTION .data\n");
    for i_var in i_vars {
        let size = match i_var.size {
            Size::Byte => "db",
            Size::Word => "dw"
        };
        let value = if let Init::Initilized(val) = i_var.init { match val { Value::Byte(v) => {v as u16}, Value::Word(v) => v } } else { unreachable!() };
        code.push_str(format!("\t{}_:\t{} {}\n", i_var.name.0, size, value).as_str());
    }
    code.push('\n');

    // section .text
    code.push_str("SECTION .text\n");
    code.push_str("\tglobal _start\n");
    code.push_str("\t_start:\n");
    for instruction in instructions {
        match instruction {
            Instruction::Assignment(assign) => {
                match assign.val {
                    ValueType::Immediate(val) => {
                        let (value, size) = match val {
                            Value::Byte(v) => (v as u16, "BYTE"),
                            Value::Word(v) => (v, "WORD")
                        };
                        code.push_str(format!("\t\tmov\t{} [{}_], {}\n", size, assign.var_name.0, value).as_str())
                    },
                    ValueType::Variable(var_name) => {
                        let var = get_variable(&variables, var_name.clone());
                        let (reg, size) = match var.size {
                            Size::Byte => ("al", "BYTE"),
                            Size::Word => ("ax", "WORD")
                        };
                        code.push_str(format!("\t\tmov\t{}, {} [{}_]\n", reg, size, var_name.0).as_str());
                        code.push_str(format!("\t\tmov\t{} [{}_], {}\n", size, assign.var_name.0, reg).as_str())
                    },
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }
    code.push('\n');

    // print!("NASM\n{}", code);
    code
}

fn get_un_init_vars(vars: &Vec<Variable>) -> (Vec<Variable>, Vec<Variable>) {
    let mut u_vars: Vec<Variable> = Vec::new();
    let mut i_vars: Vec<Variable> = Vec::new();

    for var in vars {
        match var.init {
            Init::Initilized(_) => i_vars.push(var.clone()),
            Init::Uninitilized  => u_vars.push(var.clone())
        }
    }

    (u_vars, i_vars)
}

fn get_executable_instr(instrs: Vec<Instruction>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for instr in instrs {
        match instr {
            Instruction::Variable(_) => (),
            Instruction::Assignment(_) => instructions.push(instr.clone())
        }
    }

    instructions
}

fn get_variable(vars: &Vec<Variable>, var_name: Indent) -> Variable {
    for var in vars {
        if var.name == var_name {
            return (*var).clone();
        }
    }
    unreachable!()
}