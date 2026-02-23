#!/usr/bin/env python3
import struct
import sys

OPCODES = {
    'ADD_I':   0x0001, 'SUB_I':   0x0002, 'MUL_I':   0x0003, 'DIV_I':   0x0004,
    'ADD_F':   0x0005, 'SUB_F':   0x0006, 'MUL_F':   0x0007, 'DIV_F':   0x0008,
    'IF_I':    0x0010, 'IF_F':    0x0011, 'IF_CHAR': 0x0012, 'IF_STR':  0x0013,
    'STR2INT': 0x0034, 'INT2STR': 0x0035, 'STR2F':   0x0036, 'F2STR':   0x0037,
    'STR2BOOL':0x0038, 'BOOL2STR':0x0039, 'STR2CHAR':0x003A, 'CHAR2STR':0x003B,
    'SET_OUT_FILE': 0x0051, 'PRINT_STR': 0x0052, 'READ_STR': 0x0070,
    'JMP': 0x0060, 'JMP_TRUE': 0x0061, 'JMP_FALSE': 0x0062, 'HALT': 0x0063,
}

DATA_TYPES = {'int':1, 'float':2, 'bool':3, 'string':4, 'char':5}
PROG_MAGIC, DATA_MAGIC, CODE_MAGIC = -32766, -32767, 32767

FIELD_MAP = {
    'ADD_I': (0,1,2), 'SUB_I': (0,1,2), 'MUL_I': (0,1,2), 'DIV_I': (0,1,2),
    'ADD_F': (0,1,2), 'SUB_F': (0,1,2), 'MUL_F': (0,1,2), 'DIV_F': (0,1,2),
    'IF_I': (0,1,2), 'IF_F': (0,1,2), 'IF_CHAR': (0,1,2), 'IF_STR': (0,1,2),
    'STR2INT': (0,1,None), 'INT2STR': (0,1,None),
    'STR2F': (0,1,None), 'F2STR': (0,1,None),
    'STR2BOOL': (0,1,None), 'BOOL2STR': (0,1,None),
    'STR2CHAR': (0,1,None), 'CHAR2STR': (0,1,None),
    'READ_STR': (0,None,None),
    'PRINT_STR': (None,0,None),
    'SET_OUT_FILE': (None,0,None),
    'JMP': (None,0,None),
    'JMP_TRUE': (None,0,1),
    'JMP_FALSE': (None,0,1),
    'HALT': (None,None,None),
}

def pack_value(typ, value):
    data = bytearray(29)
    if typ == 'int':      data[0:8] = struct.pack('<q', int(value))
    elif typ == 'float':  data[0:8] = struct.pack('<d', float(value))
    elif typ == 'bool':   data[0] = 1 if value else 0
    elif typ == 'string':
        s = value.encode('utf-8')
        if len(s) > 28: raise ValueError("String too long (max 28 bytes)")
        data[0:len(s)] = s
    elif typ == 'char':
        ch = str(value)
        if len(ch) != 1: raise ValueError("Char must be a single character")
        data[0:4] = struct.pack('<I', ord(ch))
    else: raise ValueError(f"Unknown type: {typ}")
    return data

def parse_line(line):
    line = line.strip()
    if not line or line.startswith(';') or line.startswith('#'):
        return None
    comment_pos = line.find(';')
    if comment_pos == -1:
        comment_pos = line.find('#')
    if comment_pos != -1:
        line = line[:comment_pos].strip()
    if not line:
        return None
    if line.endswith(':'):
        return ('label', line[:-1])
    parts = line.split()
    if not parts:
        return None
    if parts[0] == '.data' and len(parts) >= 4:
        return ('data', parts[1], int(parts[2]), ' '.join(parts[3:]))
    mnem = parts[0].upper()
    if mnem not in OPCODES:
        raise ValueError(f"Unknown mnemonic: {mnem}")
    while len(parts) < 4:
        parts.append('0')
    return ('code', mnem, parts[1], parts[2], parts[3])

def assemble(input_file, output_file):
    data_entries = []
    code_instr = []
    labels = {}
    line_num = 0
    with open(input_file, 'r', encoding='utf-8') as f:
        for line in f:
            line_num += 1
            try:
                res = parse_line(line)
                if res is None:
                    continue
                if res[0] == 'label':
                    labels[res[1]] = len(code_instr)  # номер следующей инструкции
                elif res[0] == 'data':
                    typ, reg, val = res[1], res[2], res[3]
                    packed = pack_value(typ, val)
                    data_entries.append((reg, DATA_TYPES[typ], packed))
                elif res[0] == 'code':
                    mnem, dst, src1, src2 = res[1], res[2], res[3], res[4]
                    code_instr.append((mnem, dst, src1, src2))
            except Exception as e:
                raise RuntimeError(f"Error at line {line_num}: {e}")

    code_bytes = bytearray()
    for i, (mnem, dst, src1, src2) in enumerate(code_instr):
        op = OPCODES[mnem]
        mapping = FIELD_MAP.get(mnem, (0,1,2))
        operands = [dst, src1, src2]
        fields = [0,0,0]

        def resolve(operand, ip):
            try:
                return int(operand)
            except ValueError:
                if operand in labels:
                    return labels[operand] - ip
                raise ValueError(f"Undefined label: {operand}")

        for field_idx, op_idx in enumerate(mapping):
            if op_idx is not None:
                fields[field_idx] = resolve(operands[op_idx], i)

        code_bytes.extend(struct.pack('<HHHH', op, fields[0] & 0xFFFF, fields[1] & 0xFFFF, fields[2] & 0xFFFF))

    data_bytes = bytearray()
    for reg, typ, packed in data_entries:
        data_bytes.extend(struct.pack('<H', reg))
        data_bytes.append(typ)
        data_bytes.extend(packed)

    with open(output_file, 'wb') as f:
        f.write(struct.pack('<hHI', PROG_MAGIC, len(data_entries), len(code_instr)))
        f.write(struct.pack('<h', DATA_MAGIC))
        f.write(data_bytes)
        f.write(struct.pack('<h', CODE_MAGIC))
        f.write(code_bytes)

    print(f"Assembled successfully: {len(data_entries)} data entries, {len(code_instr)} instructions")

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: python assembler.py input.asm output.bin")
        sys.exit(1)
    assemble(sys.argv[1], sys.argv[2])