use std::{
    io::stdin,
    process::exit,
};

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let code_input = buf.trim_end().split(',')
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()
        .unwrap();

    a(&code_input);
    b(&code_input);
}

fn a(code_input: &Vec<u32>) {
    let mut code = code_input.clone();
    code[1] = 12; code[2] = 2;
    run_intcode(&mut code);
    println!("a: {}", code[0])
}

fn b(code_input: &Vec<u32>) {
    let mut code = Vec::new();
    for noun in 0..=99 {
        for verb in 0..=99 {
            code.clone_from(code_input);
            code[1] = noun; code[2] = verb;
            run_intcode(&mut code);
            if code[0] == 19690720 {
                println!("b: {:2}{:02}", noun, verb);
                return;
            }
        }
    }
}

fn run_intcode(code: &mut [u32]) {
    let mut pc: usize = 0;
    
    loop {
        //eprintln!("code: {:?}\npc: {}", code, pc);
        match code[pc] {
            1 => {
                code[code[pc+3] as usize] =
                    code[code[pc+1] as usize] + code[code[pc+2] as usize];
                pc += 4;
            }
            2 => {
                code[code[pc+3] as usize] =
                    code[code[pc+1] as usize] * code[code[pc+2] as usize];
                pc += 4;
            }
            99 => return,
            c => {
                eprintln!("Invalid opcode at position {}: {}", pc, c);
                exit(1);
            }
        }
    }
}
