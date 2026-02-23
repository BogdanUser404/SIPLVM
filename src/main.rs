//Copying Bogdan Yachmenev Email: yachmenevbogdan350@gmail.com 
//GNU GPL 3.0 License

use std::env;
pub mod binary;
pub mod memory;
use crate::memory::{RegisterType, Value, get_registr, set_registr, parse_string_to_value};
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <binary file>", args[0]);
        return;
    }

    let result = binary::read_binary(&args[1]).expect("Failed to read binary");
    let (_prog_hdr, _data_hdr, data_cmds, _code_hdr, code_cmds) = result;
    let mut registers = memory::init();
    let mut else_start_index: usize = 0;
    let mut else_end_index: usize = 0;
    let mut is_if_block = false;

    // Переменная для хранения буферизованного писателя (если вывод идёт в файл)
    let mut writer: Option<BufWriter<File>> = None;

    if !data_cmds.is_empty() {
        let max_reg = data_cmds
            .iter()
            .map(|cmd| cmd.registr)
            .max()
            .unwrap() as usize;

        registers.int_registr.resize(max_reg + 1, 0);
        registers.float_registr.resize(max_reg + 1, 0.0);
        registers.bool_registr.resize(max_reg + 1, false);
        registers.string_registr.resize(max_reg + 1, String::new());
        registers.char_registr.resize(max_reg + 1, '\0');
    }

    for cmd in data_cmds {
        let reg_num = cmd.registr as usize;
        let reg_type = match cmd.data_type {
            1 => RegisterType::Int,
            2 => RegisterType::Float,
            3 => RegisterType::Bool,
            4 => RegisterType::String,
            5 => RegisterType::Char,
            _ => panic!("Unknown data type: {}", cmd.data_type),
        };
        set_registr(&mut registers, reg_type, reg_num, cmd.value);
    }

    // Исполнение кода
    let mut ip: usize = 0; // instruction pointer
    while ip < code_cmds.len() {
        let cmd = &code_cmds[ip];

        // Логика пропуска else-блока (работает только для простых if/else без вложенности)
        if !is_if_block {
            else_start_index = ip;
            else_end_index = ip;
        }
        if is_if_block {
            if ip == else_start_index {
                ip = else_end_index;
                continue;
            }
        }

        match cmd.opcode {
            0x0001 => { // ADD_I
                let val1 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg1 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg1 is not Int"),
                };
                let val2 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg2 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg2 is not Int"),
                };
                let result = val1 + val2;
                set_registr(&mut registers, RegisterType::Int, cmd.result_reg as usize, Value::Int(result));
                ip += 1;
            }
            0x0002 => { // SUB_I
                let val1 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg1 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg1 is not Int"),
                };
                let val2 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg2 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg2 is not Int"),
                };
                let result = val1 - val2;
                set_registr(&mut registers, RegisterType::Int, cmd.result_reg as usize, Value::Int(result));
                ip += 1;
            }
            0x0003 => { // MUL_I
                let val1 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg1 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg1 is not Int"),
                };
                let val2 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg2 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg2 is not Int"),
                };
                let result = val1 * val2;
                set_registr(&mut registers, RegisterType::Int, cmd.result_reg as usize, Value::Int(result));
                ip += 1;
            }
            0x0004 => { // DIV_I
                let val1 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg1 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg1 is not Int"),
                };
                let val2 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg2 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg2 is not Int"),
                };
                let result = val1 / val2;
                set_registr(&mut registers, RegisterType::Int, cmd.result_reg as usize, Value::Int(result));
                ip += 1;
            }
            0x0005 => { // ADD_F
                let val1 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg1 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg1 is not Float"),
                };
                let val2 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg2 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg2 is not Float"),
                };
                let result = val1 + val2;
                set_registr(&mut registers, RegisterType::Float, cmd.result_reg as usize, Value::Float(result));
                ip += 1;
            }
            0x0006 => { // SUB_F
                let val1 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg1 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg1 is not Float"),
                };
                let val2 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg2 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg2 is not Float"),
                };
                let result = val1 - val2;
                set_registr(&mut registers, RegisterType::Float, cmd.result_reg as usize, Value::Float(result));
                ip += 1;
            }
            0x0007 => { // MUL_F
                let val1 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg1 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg1 is not Float"),
                };
                let val2 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg2 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg2 is not Float"),
                };
                let result = val1 * val2;
                set_registr(&mut registers, RegisterType::Float, cmd.result_reg as usize, Value::Float(result));
                ip += 1;
            }
            0x0008 => { // DIV_F
                let val1 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg1 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg1 is not Float"),
                };
                let val2 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg2 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg2 is not Float"),
                };
                let result = val1 / val2;
                set_registr(&mut registers, RegisterType::Float, cmd.result_reg as usize, Value::Float(result));
                ip += 1;
            }
            0x0010 => { // IF_I
                let val1 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg1 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg1 is not Int"),
                };
                let val2 = match get_registr(&registers, RegisterType::Int, cmd.arg_reg2 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg2 is not Int"),
                };
                let result = val1 == val2;
                set_registr(&mut registers, RegisterType::Bool, cmd.result_reg as usize, Value::Bool(result));
                ip += 1;
            }
            0x0011 => { // IF_FLOAT
                let val1 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg1 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg1 is not Float"),
                };
                let val2 = match get_registr(&registers, RegisterType::Float, cmd.arg_reg2 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg2 is not Float"),
                };
                let result = val1 == val2;
                set_registr(&mut registers, RegisterType::Bool, cmd.result_reg as usize, Value::Bool(result));
                ip += 1;
            }
            0x0012 => { // IF_CHAR
                let val1 = match get_registr(&registers, RegisterType::Char, cmd.arg_reg1 as usize) {
                    Value::Char(v) => v,
                    _ => panic!("arg_reg1 is not Char"),
                };
                let val2 = match get_registr(&registers, RegisterType::Char, cmd.arg_reg2 as usize) {
                    Value::Char(v) => v,
                    _ => panic!("arg_reg2 is not Char"),
                };
                let result = val1 == val2;
                set_registr(&mut registers, RegisterType::Bool, cmd.result_reg as usize, Value::Bool(result));
                ip += 1;
            }
            0x0013 => { // IF_STRING
                let val1 = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                let val2 = match get_registr(&registers, RegisterType::String, cmd.arg_reg2 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg2 is not String"),
                };
                let result = val1 == val2;
                set_registr(&mut registers, RegisterType::Bool, cmd.result_reg as usize, Value::Bool(result));
                ip += 1;
            }
            0x0034 => { // STR_TO_INT
                let s = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                let parsed = parse_string_to_value(&s, RegisterType::Int)
                    .unwrap_or_else(|err| panic!("{} at instruction {}", err, ip));
                set_registr(&mut registers, RegisterType::Int, cmd.result_reg as usize, parsed);
                ip += 1;
            }
            0x0035 => { // INT_TO_STR
                let val = match get_registr(&registers, RegisterType::Int, cmd.arg_reg1 as usize) {
                    Value::Int(v) => v,
                    _ => panic!("arg_reg1 is not Int"),
                };
                // Используем value_to_string, если нет Display для Value
                let s = memory::value_to_string(&Value::Int(val));
                set_registr(&mut registers, RegisterType::String, cmd.result_reg as usize, Value::String(s));
                ip += 1;
            }
            0x0036 => { // STR_TO_FLOAT
                let s = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                let parsed = parse_string_to_value(&s, RegisterType::Float)
                    .unwrap_or_else(|err| panic!("{} at instruction {}", err, ip));
                set_registr(&mut registers, RegisterType::Float, cmd.result_reg as usize, parsed);
                ip += 1;
            }
            0x0037 => { // FLOAT_TO_STR
                let val = match get_registr(&registers, RegisterType::Float, cmd.arg_reg1 as usize) {
                    Value::Float(v) => v,
                    _ => panic!("arg_reg1 is not Float"),
                };
                let s = memory::value_to_string(&Value::Float(val));
                set_registr(&mut registers, RegisterType::String, cmd.result_reg as usize, Value::String(s));
                ip += 1;
            }
            0x0038 => { // STR_TO_BOOL
                let s = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                let parsed = parse_string_to_value(&s, RegisterType::Bool)
                    .unwrap_or_else(|err| panic!("{} at instruction {}", err, ip));
                set_registr(&mut registers, RegisterType::Bool, cmd.result_reg as usize, parsed);
                ip += 1;
            }
            0x0039 => { // BOOL_TO_STR
                let val = match get_registr(&registers, RegisterType::Bool, cmd.arg_reg1 as usize) {
                    Value::Bool(v) => v,
                    _ => panic!("arg_reg1 is not Bool"),
                };
                let s = memory::value_to_string(&Value::Bool(val));
                set_registr(&mut registers, RegisterType::String, cmd.result_reg as usize, Value::String(s));
                ip += 1;
            }
            0x003A => { // STR_TO_CHAR
                let s = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                let parsed = parse_string_to_value(&s, RegisterType::Char)
                    .unwrap_or_else(|err| panic!("{} at instruction {}", err, ip));
                set_registr(&mut registers, RegisterType::Char, cmd.result_reg as usize, parsed);
                ip += 1;
            }
            0x003B => { // CHAR_TO_STR
                let val = match get_registr(&registers, RegisterType::Char, cmd.arg_reg1 as usize) {
                    Value::Char(v) => v,
                    _ => panic!("arg_reg1 is not Char"),
                };
                let s = memory::value_to_string(&Value::Char(val));
                set_registr(&mut registers, RegisterType::String, cmd.result_reg as usize, Value::String(s));
                ip += 1;
            }
            0x0051 => { // SET_OUTPUT_TO_FILE
                let filename = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                let error_msg = format!("Failed to open file '{}' at instruction {}", filename, ip);
                writer = Some(BufWriter::new(File::create(&filename).expect(&error_msg)));
                ip += 1;
            }
            0x0052 => { // PRINT_STRING
                let outstring = match get_registr(&registers, RegisterType::String, cmd.arg_reg1 as usize) {
                    Value::String(v) => v,
                    _ => panic!("arg_reg1 is not String"),
                };
                if let Some(w) = &mut writer {
                    writeln!(w, "{}", outstring).expect("Failed to write to file");
                } else {
                    println!("{}", outstring);
                }
                ip += 1;
            }
            0x0060 => { // JMP
                let offset = cmd.arg_reg1 as i16;
                ip = (ip as i64 + offset as i64) as usize;
            }
            0x0061 => { // JMP_IF_TRUE
                let cond = match get_registr(&registers, RegisterType::Bool, cmd.arg_reg1 as usize) {
                    Value::Bool(v) => v,
                    _ => panic!("Condition not bool"),
                };
                let offset = cmd.arg_reg2 as i16;
                if cond {
                    ip = (ip as i64 + offset as i64) as usize;
                } else {
                    ip += 1;
                }
            }
            0x0062 => { // JMP_IF_FALSE
                let cond = match get_registr(&registers, RegisterType::Bool, cmd.arg_reg1 as usize) {
                    Value::Bool(v) => v,
                    _ => panic!("Condition not bool"),
                };
                let offset = cmd.arg_reg2 as i16;
                if !cond {
                    ip = (ip as i64 + offset as i64) as usize;
                } else {
                    ip += 1;
                }
            }
            0x0063 => { // HALT
                break; // выходим из цикла while
            }
            0x0070 => { // READ_STRING
                let input = memory::read_line_from_stdin().expect("Failed to read line");
                set_registr(&mut registers, RegisterType::String, cmd.result_reg as usize, Value::String(input));
                ip += 1;
            }
            _ => panic!("Unknown opcode: 0x{:04X}", cmd.opcode),
        }
    }

    // При выходе сбрасываем буфер, если файл был открыт
    if let Some(w) = &mut writer {
        w.flush().expect("Failed to flush file");
    }
}
