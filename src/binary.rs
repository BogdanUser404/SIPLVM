/*
 * Copyright (c) 2026 Bogdan Yachmenev <yachmenevbogdan350@gmail.com>
 * Licensed under GNU GPL v3.0 or later with ADDITIONAL TERMS:
 * 1. AI TRAINING: Use for training requires disclosure of weights & algorithms.
 * 2. MILITARY: Use for military purposes requires full source disclosure.
 * 
 * Distributed WITHOUT ANY WARRANTY. See LICENSE for full terms.
 */

use crate::memory;
use std::fs::File;
use std::io::{Read, Result, Error, ErrorKind};

// Магические числа
const PROGRAM_MAGIC: i16 = -32766;
const DATA_SECTION_MAGIC: i16 = -32767;
const CODE_SECTION_MAGIC: i16 = 32767;

/// Заголовок программы
pub struct ProgHeader {
    pub magic: i16,
    pub data_size: u16,
    pub code_size: i32,
}

/// Заголовок секции DATA
pub struct DataHeader {
    pub magic: i16,
    pub data_size: u16,
}

/// Заголовок секции CODE
pub struct CodeHeader {
    pub magic: i16,
    pub code_size: i32,
}

pub struct DataCommand {
    pub registr: u16,
    pub data_type: i8,
    pub value: memory::Value,
}

pub struct Command {
    pub opcode: u16,
    pub result_reg: u16,
    pub arg_reg1: u16,
    pub arg_reg2: u16,
}
pub struct If{
    pub len: u32,
    pub bool_registr: u16,
}
pub struct Jmp{
    pub jump_distance: i32,
}


/// Читает бинарный файл программы и возвращает все три заголовка и данные.
pub fn read_binary(path: &str) -> Result<(ProgHeader, DataHeader, Vec<DataCommand>, CodeHeader, Vec<Command>)> {
    let mut file = File::open(path)?;

    // 1. Заголовок программы (8 байт)
    let mut prog_buf = [0u8; 8];
    file.read_exact(&mut prog_buf)?;
    let prog_magic = i16::from_le_bytes([prog_buf[0], prog_buf[1]]);
    let data_size = u16::from_le_bytes([prog_buf[2], prog_buf[3]]);
    let code_size = i32::from_le_bytes([prog_buf[4], prog_buf[5], prog_buf[6], prog_buf[7]]);

    if prog_magic != PROGRAM_MAGIC {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid program magic"));
    }
    let prog_header = ProgHeader {
        magic: prog_magic,
        data_size,
        code_size,
    };

    // 2. Заголовок секции DATA (2 байта)
    let mut data_magic_buf = [0u8; 2];
    file.read_exact(&mut data_magic_buf)?;
    let data_magic = i16::from_le_bytes(data_magic_buf);
    if data_magic != DATA_SECTION_MAGIC {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid DATA section magic"));
    }
    let data_header = DataHeader { magic: data_magic, data_size };

    // 3. Чтение записей DATA
    let mut data_commands = Vec::with_capacity(data_size as usize);
    for _ in 0..data_size {
        let mut entry = [0u8; 32];
        file.read_exact(&mut entry)?;

        let registr = u16::from_le_bytes([entry[0], entry[1]]);
        let data_type = entry[2] as i8;
        let value_bytes = &entry[3..]; // 29 байт

        let value = match data_type {
            1 => {
                let int_val = i64::from_le_bytes(value_bytes[0..8].try_into().unwrap());
                memory::Value::Int(int_val)
            }
            2 => {
                let float_val = f64::from_le_bytes(value_bytes[0..8].try_into().unwrap());
                memory::Value::Float(float_val)
            }
            3 => {
                let bool_val = value_bytes[0] != 0;
                memory::Value::Bool(bool_val)
            }
            4 => {
                let len = value_bytes.iter().position(|&b| b == 0).unwrap_or(29);
                let s = String::from_utf8_lossy(&value_bytes[0..len]).to_string();
                memory::Value::String(s)
            }
            5 => {
                let char_val = u32::from_le_bytes(value_bytes[0..4].try_into().unwrap());
                let c = char::from_u32(char_val).unwrap_or('?');
                memory::Value::Char(c)
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unknown data type")),
        };

        data_commands.push(DataCommand {
            registr,
            data_type,
            value,
        });
    }

    // 4. Заголовок секции CODE (2 байта)
    let mut code_magic_buf = [0u8; 2];
    file.read_exact(&mut code_magic_buf)?;
    let code_magic = i16::from_le_bytes(code_magic_buf);
    if code_magic != CODE_SECTION_MAGIC {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid CODE section magic"));
    }
    let code_header = CodeHeader { magic: code_magic, code_size };

    // 5. Чтение инструкций CODE
    let mut commands = Vec::with_capacity(code_size as usize);
    for _ in 0..code_size {
        let mut instr = [0u8; 8];
        file.read_exact(&mut instr)?;

        let opcode = u16::from_le_bytes([instr[0], instr[1]]);
        let result_reg = u16::from_le_bytes([instr[2], instr[3]]);
        let arg_reg1 = u16::from_le_bytes([instr[4], instr[5]]);
        let arg_reg2 = u16::from_le_bytes([instr[6], instr[7]]);

        commands.push(Command {
            opcode,
            result_reg,
            arg_reg1,
            arg_reg2,
        });
    }

    Ok((prog_header, data_header, data_commands, code_header, commands))
}
