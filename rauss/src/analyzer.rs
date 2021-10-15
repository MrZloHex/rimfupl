use crate::types::*;


pub fn analyze_instr(instructions_p: &Vec<Instruction>) -> bool {
    let variables = get_vars(instructions_p);
    let assignments = get_assign(instructions_p);
    let uninit_vars = get_uninit_vars(&variables, &assignments);

    // check for unique names of variables
    let mut tmp_is: Vec<Indent> = Vec::new();
    for variable in &variables {
        if !tmp_is.is_empty() {
            for tmp_i in &tmp_is {
                if *tmp_i == variable.name {
                    error(1, variable.name.clone());
                }
            }
        }
        tmp_is.push(variable.name.clone())
    }
    drop(tmp_is);


    // check for right size of variable
    for variable in &variables {
        match variable.init {
            Init::Initilized(value) => {
                match value {
                    Value::Byte(_) => if variable.size != Size::Byte { error(0, variable) },
                    Value::Word(_) => if variable.size != Size::Word { error(0, variable) }
                }
            },
            Init::Uninitilized => ()
        }
    }

    // check for assignment to existing and initilized variable
    

    // check for correct size of operands of assignment
    for assignment in &assignments {
        match &assignment.val {
            ValueType::Immediate(val) => {
                let size_var = get_size_var(&variables, assignment.var_name.clone());
                match val {
                    Value::Byte(_) => if size_var != Size::Byte { error(2, assignment.var_name.clone()) },
                    Value::Word(_) => if size_var != Size::Word { error(2, assignment.var_name.clone()) }
                }
            },
            ValueType::Variable(var) => {
                let size_var = get_size_var(&variables, assignment.var_name.clone());
                let size_val = get_size_var(&variables, (*var).clone());
                if size_var != size_val {
                    error(2, assignment.var_name.clone())
                }
            },
            _ => unreachable!()
        }
    }
    



    true
}


/*  Error codes:
 *
 *  - 0: Size of variable not corresponds to it's value
 *  - 1: Variable name is already used
 *  - 2: Assignment to different sizes
 *
 */

fn error<T>(error_code: u8, problem_struct: T) -> ! 
where T: std::fmt::Debug
{
    match error_code {
        0 => eprintln!("Size of variable not corresponds to its value,\nsee variable {:?}", problem_struct),
        1 => eprintln!("Variable name `{:?}` is already used", problem_struct),
        2 => eprintln!("Assigning to `{:?}` diiferent size value", problem_struct),
        _ => unreachable!()
    }
    std::process::exit(1);
}


fn get_vars(instructions_p: &Vec<Instruction>) -> Vec<Variable> {
    let mut variables: Vec<Variable> = Vec::new();
    for instruction in instructions_p {
        if matches!(instruction, Instruction::Variable(_)) {
            let variable = if let Instruction::Variable(var) = instruction { var } else { unreachable!() };
            variables.push((*variable).clone());
        }
    }
    variables
}


fn get_assign(instructions_p: &Vec<Instruction>) -> Vec<Assignment> {
    let mut assignments: Vec<Assignment> = Vec::new();
    for instruction in instructions_p {
        if matches!(instruction, Instruction::Assignment(_)) {
            let assignment = if let Instruction::Assignment(ass) = instruction { ass } else { unreachable!() };
            assignments.push((*assignment).clone())
        }
    }
    assignments
}

fn get_size_var(vars: &Vec<Variable>, var_name: Indent) -> Size {
    for var in vars {
        if var.name == var_name {
            return var.size.clone()
        }
    }
    unreachable!();
}

