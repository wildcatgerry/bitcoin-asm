extern crate app;

pub mod config;

use std::fs::File;
use std::io::prelude::*;


fn command_to_opcode(command: &str) -> u8 {
    match command {	
        "OP_0" => 0,
        "OP_FALSE" => 0,
        "OP_PUSHDATA1" => 76,
        "OP_PUSHDATA2" => 77,
        "OP_PUSHDATA4" => 78,
        "OP_1NEGATE" => 79,
        "OP_1" => 81,
        "OP_TRUE" => 81,
        "OP_2" => 82,
        "OP_3" => 83,
        "OP_4" => 84,
        "OP_5" => 85,
        "OP_6" => 86,
        "OP_7" => 87,
        "OP_8" => 88,
        "OP_9" => 89,
        "OP_10" => 90,
        "OP_11" => 91,
        "OP_12" => 92,
        "OP_13" => 93,
        "OP_14" => 94,
        "OP_15" => 95,
        "OP_16" => 96,
        "OP_NOP" => 97,
        "OP_IF" => 99,
        "OP_NOTIF" => 100,
        "OP_ELSE" => 103,
        "OP_ENDIF" => 104,
        "OP_VERIFY" => 105,
        "OP_RETURN" => 106,
        "OP_TOALTSTACK" => 107,
        "OP_FROMALTSTACK" => 108,
        "OP_IFDUP" => 115,
        "OP_DEPTH" => 116,
        "OP_DROP" => 117,
        "OP_DUP" => 118,
        "OP_NIP" => 119,
        "OP_OVER" => 120,
        "OP_PICK" => 121,
        "OP_ROLL" => 122,
        "OP_ROT" => 123,
        "OP_SWAP" => 124,
        "OP_TUCK" => 125,
        "OP_2DROP" => 109,
        "OP_2DUP" => 110,
        "OP_3DUP" => 111,
        "OP_2OVER" => 112,
        "OP_2ROT" => 113,
        "OP_2SWAP" => 114,
        "OP_SIZE" => 130,
        "OP_EQUAL" => 135,
        "OP_EQUALVERIFY" => 136,
        "OP_1ADD" => 139,
        "OP_1SUB" => 140,
        "OP_NEGATE" => 143,
        "OP_ABS" => 144,
        "OP_NOT" => 145,
        "OP_0NOTEQUAL" => 146,
        "OP_ADD" => 147,
        "OP_SUB" => 148,
        "OP_BOOLAND" => 154,
        "OP_BOOLOR" => 155,
        "OP_NUMEQUAL" => 156,
        "OP_NUMEQUALVERIFY" => 157,
        "OP_NUMNOTEQUAL" => 158,
        "OP_LESSTHAN" => 159,
        "OP_GREATERTHAN" => 160,
        "OP_LESSTHANOREQUAL" => 161,
        "OP_GREATERTHANOREQUAL" => 162,
        "OP_MIN" => 163,
        "OP_MAX" => 164,
        "OP_WITHIN" => 165,
        "OP_RIPEMD160" => 166,
        "OP_SHA1" => 167,
        "OP_SHA256" => 168,
        "OP_HASH160" => 169,
        "OP_HASH256" => 170,
        "OP_CODESEPARATOR" => 171,
        "OP_CHECKSIG" => 172,
        "OP_CHECKSIGVERIFY" => 173,
        "OP_CHECKMULTISIG" => 174,
        "OP_CHECKMULTISIGVERIFY" => 175,
        "OP_PUBKEYHASH" => 253,
        "OP_PUBKEY" => 254,
        "OP_INVALIDOPCODE" => 255,
        "OP_RESERVED" => 80,
        "OP_VER" => 98,
        "OP_VERIF" => 101,
        "OP_VERNOTIF" => 102,
        "OP_RESERVED1" => 137,
        "OP_RESERVED2" => 138,
        "OP_NOP1" => 176,
        "OP_NOP2" => 177,
        "OP_NOP3" => 178,
        "OP_NOP4" => 179,
        "OP_NOP5" => 180,
        "OP_NOP6" => 181,
        "OP_NOP7" => 182,
        "OP_NOP8" => 183,
        "OP_NOP9" => 184,
        "OP_NOP10" => 185,
        "OP_CAT" | "OP_SUBSTR" | "OP_OP_LEFT" | "OP_RIGHT" | "OP_INVERT" | "OP_AND" | "OP_OR" | "OP_XOR" | "OP_2MUL" | "OP_2DIV" | "OP_MUL" | "OP_DIV" | "OP_MOD" | "OP_LSHIFT" | "OP_RSHIFT" => panic!(format!("INVALID OP CODE INCLUDED: {}", command)),
        _ => u8::from_str_radix(command, 16).expect(&format!("Failed to convert {:?} to u8", command))
    }
}

fn opcode_to_command<'a>(opcode: u8) -> &'a str {
    match opcode  {
        0 => "OP_FALSE",
        76 => "OP_PUSHDATA1",
        77 => "OP_PUSHDATA2",
        78 => "OP_PUSHDATA4",
        79 => "OP_1NEGATE",
        81 => "OP_1",
        82 => "OP_2",
        83 => "OP_3",
        84 => "OP_4",
        85 => "OP_5",
        86 => "OP_6",
        87 => "OP_7",
        88 => "OP_8",
        89 => "OP_9",
        90 => "OP_10",
        91 => "OP_11",
        92 => "OP_12",
        93 => "OP_13",
        94 => "OP_14",
        95 => "OP_15",
        96 => "OP_16",
        97 => "OP_NOP",
        99 => "OP_IF",
        100 => "OP_NOTIF",
        103 => "OP_ELSE",
        104 => "OP_ENDIF",
        105 => "OP_VERIFY",
        106 => "OP_RETURN",
        107 => "OP_TOALTSTACK",
        108 => "OP_FROMALTSTACK",
        115 => "OP_IFDUP",
        116 => "OP_DEPTH",
        117 => "OP_DROP",
        118 => "OP_DUP",
        119 => "OP_NIP",
        120 => "OP_OVER",
        121 => "OP_PICK",
        122 => "OP_ROLL",
        123 => "OP_ROT",
        124 => "OP_SWAP",
        125 => "OP_TUCK",
        109 => "OP_2DROP",
        110 => "OP_2DUP",
        111 => "OP_3DUP",
        112 => "OP_2OVER",
        113 => "OP_2ROT",
        114 => "OP_2SWAP",
        135 => "OP_EQUAL",
        136 => "OP_EQUALVERIFY",
        139 => "OP_1ADD",
        140 => "OP_1SUB",
        143 => "OP_NEGATE",
        144 => "OP_ABS",
        145 => "OP_NOT",
        146 => "OP_0NOTEQUAL",
        147 => "OP_ADD",
        148 => "OP_SUB",
        154 => "OP_BOOLAND",
        155 => "OP_BOOLOR",
        156 => "OP_NUMEQUAL",
        157 => "OP_NUMEQUALVERIFY",
        158 => "OP_NUMNOTEQUAL",
        159 => "OP_LESSTHAN",
        160 => "OP_GREATERTHAN",
        161 => "OP_LESSTHANOREQUAL",
        162 => "OP_GREATERTHANOREQUAL",
        163 => "OP_MIN",
        164 => "OP_MAX",
        165 => "OP_WITHIN",
        166 => "OP_RIPEMD160",
        167 => "OP_SHA1",
        168 => "OP_SHA256",
        169 => "OP_HASH160",
        170 => "OP_HASH256",
        171 => "OP_CODESEPARATOR",
        172 => "OP_CHECKSIG",
        173 => "OP_CHECKSIGVERIFY",
        174 => "OP_CHECKMULTISIG",
        175 => "OP_CHECKMULTISIGVERIFY",
        253 => "OP_PUBKEYHASH",
        254 => "OP_PUBKEY",
        255 => "OP_INVALIDOPCODE",
        80 => "OP_RESERVED",
        98 => "OP_VER",
        101 => "OP_VERIF",
        102 => "OP_VERNOTIF",
        137 => "OP_RESERVED1",
        138 => "OP_RESERVED2",
        176 => "OP_NOP1",
        177 => "OP_NOP2",
        178 => "OP_NOP3",
        179 => "OP_NOP4",
        180 => "OP_NOP5",
        181 => "OP_NOP6",
        182 => "OP_NOP7",
        183 => "OP_NOP8",
        184 => "OP_NOP9",
        185 => "OP_NOP10",
        _ => panic!("Not an opcode")
    }
}

pub fn assemble(conf: config::Config) {
    let body = load_file(&conf);

    let hex_buffer = assemble_internal(body);

    write_file(&conf, hex_buffer);
}

fn assemble_internal(body: String) -> String {
    let mut hex_buffer = String::new();

    for token in body.split_whitespace() {
        if token.starts_with("OP_") {
            hex_buffer.push_str(&format!("{:X}", command_to_opcode(token)));
        } 
        else {
            hex_buffer.push_str(&token);
        }
    }

    hex_buffer
}

fn load_file(conf: &config::Config) -> String {
    let mut infile = File::open(&conf.input).expect("File not found.");

    let mut body = String::new();
    infile.read_to_string(&mut body).expect("Could not read file.");
    body
}

fn write_file(conf: &config::Config, body: String) {	
    let mut outfile = File::create(&conf.output).expect("Could not open file.");
    outfile.write_all(body.as_bytes()).expect("Could not write file.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        let body = "OP_RETURN";
        assert_eq!("6A".to_string(), assemble_internal(body.to_string()));
    }

    #[test]
    fn test_sample() {
        let body = "OP_DUP 
OP_HASH160 
788464014149d93b4a6135f3d665a0a2d743e6c3 
OP_EQUALVERIFY 
OP_CHECKSIG";
        assert_eq!("76A9788464014149d93b4a6135f3d665a0a2d743e6c388AC".to_string(), assemble_internal(body.to_string()));
    }
}
