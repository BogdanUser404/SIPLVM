/*
 * Copyright (c) 2026 Bogdan Yachmenev <yachmenevbogdan350@gmail.com>
 * Licensed under GNU GPL v3.0 or later with ADDITIONAL TERMS:
 * 1. AI TRAINING: Use for training requires disclosure of weights & algorithms.
 * 2. MILITARY: Use for military purposes requires full source disclosure.
 * 
 * Distributed WITHOUT ANY WARRANTY. See LICENSE for full terms.
 */

pub struct Registers {
    pub int_registr: Vec<i64>,
    pub float_registr: Vec<f64>,
    pub string_registr: Vec<String>,
    pub bool_registr: Vec<bool>,
    pub char_registr: Vec<char>,
}

pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    Null,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterType {
    Int,
    Float,
    Bool,
    String,
    Char,
}

pub fn get_registr(registers: &Registers, reg_type: RegisterType, index: usize) -> Value {
    if index >= registers.int_registr.len() {
        panic!("Register index {} out of bounds (max {})", index, registers.int_registr.len());
    }
    match reg_type {
        RegisterType::Int => Value::Int(registers.int_registr[index]),
        RegisterType::Float => Value::Float(registers.float_registr[index]),
        RegisterType::Bool => Value::Bool(registers.bool_registr[index]),
        RegisterType::String => Value::String(registers.string_registr[index].clone()),
        RegisterType::Char => Value::Char(registers.char_registr[index]),
    }
}

pub fn set_registr(registers: &mut Registers, reg_type: RegisterType, index: usize, value: Value) {
    // Расширяем векторы, если индекс выходит за пределы
    if index >= registers.int_registr.len() {
        let new_len = index + 1;
        registers.int_registr.resize(new_len, 0);
        registers.float_registr.resize(new_len, 0.0);
        registers.bool_registr.resize(new_len, false);
        registers.string_registr.resize(new_len, String::new());
        registers.char_registr.resize(new_len, '\0');
    }
    // Проверяем соответствие типа и записываем
    match (reg_type, value) {
        (RegisterType::Int, Value::Int(v)) => registers.int_registr[index] = v,
        (RegisterType::Float, Value::Float(v)) => registers.float_registr[index] = v,
        (RegisterType::Bool, Value::Bool(v)) => registers.bool_registr[index] = v,
        (RegisterType::String, Value::String(v)) => registers.string_registr[index] = v,
        (RegisterType::Char, Value::Char(v)) => registers.char_registr[index] = v,
        _ => panic!("Type mismatch: expected {:?}, got different value", reg_type),
    }
}

pub fn init() -> Registers {
    Registers {
        int_registr: Vec::new(),
        float_registr: Vec::new(),
        string_registr: Vec::new(),
        bool_registr: Vec::new(),
        char_registr: Vec::new(),
    }
}

// Вспомогательная функция для преобразования Value в строку (если Display не реализован)
pub fn value_to_string(v: &Value) -> String {
    match v {
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Char(c) => c.to_string(),
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
    }
}

// Парсит строку в значение указанного типа.
pub fn parse_string_to_value(s: &str, target_type: RegisterType) -> Result<Value, String> {
    match target_type {
        RegisterType::Int => s.trim().parse::<i64>()
            .map(Value::Int)
            .map_err(|_| format!("Cannot parse '{}' as integer", s)),
        RegisterType::Float => s.trim().parse::<f64>()
            .map(Value::Float)
            .map_err(|_| format!("Cannot parse '{}' as float", s)),
        RegisterType::Bool => {
            let trimmed = s.trim().to_lowercase();
            if trimmed == "true" || trimmed == "1" {
                Ok(Value::Bool(true))
            } else if trimmed == "false" || trimmed == "0" {
                Ok(Value::Bool(false))
            } else {
                Err(format!("Cannot parse '{}' as boolean (expected true/false/1/0)", s))
            }
        }
        RegisterType::Char => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                Err("Empty string cannot be parsed as char".to_string())
            } else {
                Ok(Value::Char(trimmed.chars().next().unwrap()))
            }
        }
        RegisterType::String => Ok(Value::String(s.to_string())),
    }
}

// Читает строку из стандартного ввода (без символа перевода строки).
pub fn read_line_from_stdin() -> std::io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    Ok(input)
}
