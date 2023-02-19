use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let data = fs::read_to_string(args[1].clone()).expect("Unable to read file").replace("\r", "");
    let lines: Vec<&str> = data.split("\n").collect();

    let mut functions: HashMap<&str, u16> = HashMap::new();
    let mut function_number: u16 = 0;

    let mut binary: Vec<u8> = Vec::new();

    for line in lines {
        let arg: Vec<&str> = line.split(" ").collect();

        match arg[0] {
            "LDV" => {
                binary.push(0b0000_0000);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "LDM" => {
                binary.push(0b0000_0001);
                binary.push(arg[1].parse::<u8>().unwrap());
                let num = arg[2].parse::<u16>().unwrap();
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
            }
            "STM" => {
                binary.push(0b0000_0010);
                binary.push(arg[1].parse::<u8>().unwrap());
                let num = arg[2].parse::<u16>().unwrap();
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
            }
            "MOV" => {
                binary.push(0b0000_0011);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "LDVZ" => {
                binary.push(0b0000_0100);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "LVNZ" => {
                binary.push(0b0000_0101);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "PUSH" => {
                binary.push(0b0000_00110);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "PSHV" => {
                binary.push(0b0000_00111);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "POP" => {
                binary.push(0b0000_01000);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "PPM" => {
                binary.push(0b0000_01001);
                binary.push((arg[1].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[1].parse::<u16>().unwrap() as u8);
            }
            "OSTM" => {
                binary.push(0b0000_1010);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "OLDM" => {
                binary.push(0b0000_1011);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "STVM" => {
                binary.push(0b0000_1100);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
            }

            "ADD" => {
                binary.push(0b0001_0000);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "ADT" => {
                binary.push(0b0001_0001);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "SUB" => {
                binary.push(0b0001_0010);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "SBT" => {
                binary.push(0b0001_0011);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "ADV" => {
                binary.push(0b0001_0100);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "ADVT" => {
                binary.push(0b0001_0101);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "SBV" => {
                binary.push(0b0001_0110);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "SBVT" => {
                binary.push(0b0001_0111);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
            }
            "INC" => {
                binary.push(0b0001_1000);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "DEC" => {
                binary.push(0b0001_1001);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "ADC" => {
                binary.push(0b0001_1010);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "CCF" => {
                binary.push(0b0001_1011);
            }
            "SCF" => {
                binary.push(0b0001_1100);
            }
            "SBC" => {
                binary.push(0b0001_1101);
                binary.push(arg[1].parse::<u8>().unwrap());
            }

            "JMP" => {
                binary.push(0b0011_0000);
                binary.push((arg[1].parse::<i8>().unwrap()) as u8);
            }
            "JPZ" => {
                binary.push(0b0011_0001);
                binary.push((arg[1].parse::<i8>().unwrap()) as u8);
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "JPNZ" => {
                binary.push(0b0011_0010);
                binary.push((arg[1].parse::<i8>().unwrap()) as u8);
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "FNC" => {
                functions.insert(arg[1], function_number);
                binary.push(0b0011_0011);
                binary.push((function_number >> 8) as u8);
                binary.push(function_number as u8);
                function_number += 1;
            }
            "RET" => {
                binary.push(0b0011_0100);
            }
            "CAL" => {
                let num = functions.get(arg[1]).unwrap().clone();
                binary.push(0b0011_0101);
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
            }
            "CLZ" => {
                let num = functions.get(arg[1]).unwrap().clone();
                binary.push(0b0011_0110);
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
                binary.push((arg[2].parse::<i8>().unwrap()) as u8);
            }
            "CLNZ" => {
                let num = functions.get(arg[1]).unwrap().clone();
                binary.push(0b0011_0111);
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
                binary.push((arg[2].parse::<i8>().unwrap()) as u8);
            }
            
            "MLT" => {
                binary.push(0b0100_0000);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
                binary.push(arg[4].parse::<u8>().unwrap());
            }
            "DIV" => {
                binary.push(0b0100_0001);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push(arg[3].parse::<u8>().unwrap());
                binary.push(arg[4].parse::<u8>().unwrap());
                binary.push(arg[5].parse::<u8>().unwrap());
                binary.push(arg[6].parse::<u8>().unwrap());
            }

            "STD" => {
                let num = arg[1].parse::<u32>().unwrap();
                binary.push(0b0110_0000);
                binary.push((num >> 24) as u8);
                binary.push((num >> 16) as u8);
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "SVD" => {
                let num = arg[1].parse::<u32>().unwrap();
                binary.push(0b0110_0001);
                binary.push((num >> 24) as u8);
                binary.push((num >> 16) as u8);
                binary.push((num >> 8) as u8);
                binary.push(num as u8);
                binary.push(arg[2].parse::<u8>().unwrap());
            }
            "LDTM" => {
                let drive = arg[1].parse::<u32>().unwrap();
                let mem = arg[2].parse::<u32>().unwrap();
                binary.push(0b0110_0010);
                binary.push((drive >> 24) as u8);
                binary.push((drive >> 16) as u8);
                binary.push((drive >> 8) as u8);
                binary.push(drive as u8);
                binary.push((mem >> 16) as u8);
                binary.push((mem >> 8) as u8);
                binary.push(mem as u8);
            }
            "LDML" => {
                let drive = arg[1].parse::<u32>().unwrap();
                let mem = arg[2].parse::<u32>().unwrap();
                let length = arg[3].parse::<u32>().unwrap();
                binary.push(0b0110_0011);
                binary.push((drive >> 24) as u8);
                binary.push((drive >> 16) as u8);
                binary.push((drive >> 8) as u8);
                binary.push(drive as u8);
                binary.push((mem >> 16) as u8);
                binary.push((mem >> 8) as u8);
                binary.push(mem as u8);
                binary.push((length >> 8) as u8);
                binary.push(length as u8);
            }
            "SDML" => {
                let drive = arg[1].parse::<u32>().unwrap();
                let mem = arg[2].parse::<u32>().unwrap();
                let length = arg[3].parse::<u32>().unwrap();
                binary.push(0b0110_0100);
                binary.push((drive >> 24) as u8);
                binary.push((drive >> 16) as u8);
                binary.push((drive >> 8) as u8);
                binary.push(drive as u8);
                binary.push((mem >> 16) as u8);
                binary.push((mem >> 8) as u8);
                binary.push(mem as u8);
                binary.push((length >> 8) as u8);
                binary.push(length as u8);
            }

            "WRT" => {
                binary.push(0b1000_0000);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push(arg[5].parse::<u8>().unwrap());
                binary.push(arg[6].parse::<u8>().unwrap());
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
            }
            "WRTL" => {
                binary.push(0b1000_0001);

                let chars: Vec<char> = arg[1]
                    .replace("\\s", "α")
                    .chars()
                    .collect();
                let mut binary_text: Vec<u8> = Vec::new();
                
                for char in chars {
                    binary_text.push(char_to_id(&char.to_string()));
                }
                
                binary.push(binary_text.len() as u8);
                binary.append(&mut binary_text);

                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push(arg[5].parse::<u8>().unwrap());
                binary.push(arg[6].parse::<u8>().unwrap());
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
            }
            "DRWR" => {
                binary.push(0b1000_0010);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push(arg[5].parse::<u8>().unwrap());
                binary.push(arg[6].parse::<u8>().unwrap());
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
            }
            "DRWC" => {
                binary.push(0b1000_0011);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push(arg[4].parse::<u8>().unwrap());
                binary.push(arg[5].parse::<u8>().unwrap());
                binary.push(arg[6].parse::<u8>().unwrap());
                binary.push(arg[7].parse::<u8>().unwrap());
            }
            "MWRT" => {
                binary.push(0b1000_0101);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push((arg[5].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[5].parse::<u16>().unwrap() as u8);
                binary.push((arg[6].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[6].parse::<u16>().unwrap() as u8);
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
                binary.push(arg[9].parse::<u8>().unwrap());
                binary.push(arg[10].parse::<u8>().unwrap());
            }
            "MWRTL" => {
                binary.push(0b1000_0110);

                let chars: Vec<char> = arg[1]
                    .replace("\\s", "α")
                    .chars()
                    .collect();
                let mut binary_text: Vec<u8> = Vec::new();
                
                for char in chars {
                    binary_text.push(char_to_id(&char.to_string()));
                }
                
                binary.push(binary_text.len() as u8);
                binary.append(&mut binary_text);

                binary.push(arg[2].parse::<u8>().unwrap());
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push((arg[5].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[5].parse::<u16>().unwrap() as u8);
                binary.push((arg[6].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[6].parse::<u16>().unwrap() as u8);
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
                binary.push(arg[9].parse::<u8>().unwrap());
                binary.push(arg[10].parse::<u8>().unwrap());
            }
            "MDRWR" => {
                binary.push(0b1000_0111);
                binary.push((arg[1].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[1].parse::<u16>().unwrap() as u8);
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push((arg[5].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[5].parse::<u16>().unwrap() as u8);
                binary.push((arg[6].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[6].parse::<u16>().unwrap() as u8);
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
                binary.push(arg[9].parse::<u8>().unwrap());
                binary.push(arg[10].parse::<u8>().unwrap());
            }
            "MDRWC" => {
                binary.push(0b1000_1000);
                binary.push(arg[1].parse::<u8>().unwrap());
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push((arg[5].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[5].parse::<u16>().unwrap() as u8);
                binary.push(arg[6].parse::<u8>().unwrap());
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
                binary.push(arg[9].parse::<u8>().unwrap());
            }
            "DRWL" => {
                binary.push(0b1000_1001);
                binary.push((arg[1].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[1].parse::<u16>().unwrap() as u8);
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push(arg[5].parse::<u8>().unwrap());
                binary.push(arg[6].parse::<u8>().unwrap());
                binary.push(arg[7].parse::<u8>().unwrap());
                binary.push(arg[8].parse::<u8>().unwrap());
                binary.push(arg[9].parse::<u8>().unwrap());
            }
            "MDRWL" => {
                binary.push(0b1000_1010);
                binary.push((arg[1].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[1].parse::<u16>().unwrap() as u8);
                binary.push((arg[2].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[2].parse::<u16>().unwrap() as u8);
                binary.push((arg[3].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[3].parse::<u16>().unwrap() as u8);
                binary.push((arg[4].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[4].parse::<u16>().unwrap() as u8);
                binary.push((arg[5].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[5].parse::<u16>().unwrap() as u8);
                binary.push((arg[6].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[6].parse::<u16>().unwrap() as u8);
                binary.push((arg[7].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[7].parse::<u16>().unwrap() as u8);
                binary.push((arg[8].parse::<u16>().unwrap() >> 8) as u8);
                binary.push(arg[8].parse::<u16>().unwrap() as u8);
                binary.push(arg[9].parse::<u8>().unwrap());
                binary.push(arg[10].parse::<u8>().unwrap());
                binary.push(arg[11].parse::<u8>().unwrap());
                binary.push(arg[12].parse::<u8>().unwrap());
                binary.push(arg[13].parse::<u8>().unwrap());
            }

            "CLS" => {
                binary.push(0b1000_0100);
            }

            "EXT" => {
                binary.push(0b1111_1110);
                binary.push(arg[1].parse::<u8>().unwrap());
            }
            "HLT" => {
                binary.push(0b1111_1111);
            }

            _ => {}
        }
    }

    println!("Size: {} Bytes", binary.len());
    /*for byte in binary.clone() {
        println!("{:8b}", byte);
    }*/
    fs::write(args[2].clone(), binary).expect("Unable to write file");
}

fn char_to_id(str: &str) -> u8 {
    match str {
        "α"=>return 0b0101_1011,

        "A" => return 0b0000_0000,
        "B" => return 0b0000_0001,
        "C" => return 0b0000_0010,
        "D" => return 0b0000_0011,
        "E" => return 0b0000_0100,
        "F" => return 0b0000_0101,
        "G" => return 0b0000_0110,
        "H" => return 0b0000_0111,
        "I" => return 0b0000_1000,
        "J" => return 0b0000_1001,
        "K" => return 0b0000_1010,
        "L" => return 0b0000_1011,
        "M" => return 0b0000_1100,
        "N" => return 0b0000_1101,
        "O" => return 0b0000_1110,
        "P" => return 0b0000_1111,

        "Q" => return 0b0001_0000,
        "R" => return 0b0001_0001,
        "S" => return 0b0001_0010,
        "T" => return 0b0001_0011,
        "U" => return 0b0001_0100,
        "V" => return 0b0001_0101,
        "W" => return 0b0001_0110,
        "X" => return 0b0001_0111,
        "Y" => return 0b0001_1000,
        "Z" => return 0b0001_1001,
        "<" => return 0b0001_1010,
        ">" => return 0b0001_1011,
        "(" => return 0b0001_1100,
        ")" => return 0b0001_1101,
        "[" => return 0b0001_1110,
        "]" => return 0b0001_1111,

        "a" => return 0b0010_0000,
        "b" => return 0b0010_0001,
        "c" => return 0b0010_0010,
        "d" => return 0b0010_0011,
        "e" => return 0b0010_0100,
        "f" => return 0b0010_0101,
        "g" => return 0b0010_0110,
        "h" => return 0b0010_0111,
        "i" => return 0b0010_1000,
        "j" => return 0b0010_1001,
        "k" => return 0b0010_1010,
        "l" => return 0b0010_1011,
        "m" => return 0b0010_1100,
        "n" => return 0b0010_1101,
        "o" => return 0b0010_1110,
        "p" => return 0b0010_1111,

        "q" => return 0b0011_0000,
        "r" => return 0b0011_0001,
        "s" => return 0b0011_0010,
        "t" => return 0b0011_0011,

        "u" => return 0b0011_0101,
        "v" => return 0b0011_0110,
        "w" => return 0b0011_0111,
        "x" => return 0b0011_1000,
        "y" => return 0b0011_1001,
        "z" => return 0b0011_1010,
        "!" => return 0b0011_1011,
        "?" => return 0b0011_1100,
        "|" => return 0b0011_1101,
        "&" => return 0b0011_1110,
        "#" => return 0b0011_1111,

        "0" => return 0b0100_0000,
        "1" => return 0b0100_0001,
        "2" => return 0b0100_0010,
        "3" => return 0b0100_0011,
        "4" => return 0b0100_0100,
        "5" => return 0b0100_0101,
        "6" => return 0b0100_0110,
        "7" => return 0b0100_0111,
        "8" => return 0b0100_1000,
        "9" => return 0b0100_1001,
        "/" => return 0b0100_1010,
        "\\"=> return 0b0100_1011,
        "~" => return 0b0100_1100,
        "_" => return 0b0100_1101,
        "-" => return 0b0100_1110,
        "+" => return 0b0100_1111,

        "*" => return 0b0101_0000,
        "." => return 0b0101_0001,
        "," => return 0b0101_0010,
        ";" => return 0b0101_0011,
        "=" => return 0b0101_0100,
        "%" => return 0b0101_0101,
        "{" => return 0b0101_0110,
        "}" => return 0b0101_0111,
        ":" => return 0b0101_1000,
        "\""=> return 0b0101_1001,
        "'" => return 0b0101_1010,
        
        "■" => return 0b0101_1100,
        "■" => return 0b0101_1101,
        "■" => return 0b0101_1110,
        "■" => return 0b0101_1111,

        _ => return 0b1111_0000
    }
}
