use macroquad::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::{thread, time};

#[derive(Debug, Clone)]
struct Cpu {
    r: [u8; 8],
    carry_flag: bool,
    
    mem_ptr: usize,
    stack_ptr: usize,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            r: [0; 8],
            carry_flag: false,
            
            mem_ptr: 256 * 256,
            stack_ptr: 512 * 1024,
        }
    }
}

static mut RAM: [u8; 512 * 1024] = [0; 512 * 1024];
static mut DRIVE: Vec<u8> = Vec::new();

fn debug(cpu: Cpu, functions: HashMap<u16, usize>) {
    println!("{:?}, Functions: {:?}", cpu, functions);
}

#[macroquad::main("VM")]
async fn main() {
    let mut cpu = Cpu::new();
    let mut functions = HashMap::new();
    let mut called_from: Vec<usize> = Vec::new();

    let font = load_ttf_font_from_bytes(include_bytes!("VT323-Regular.ttf")).unwrap();

    thread::spawn(move || drive_saver());

    unsafe {
        DRIVE = fs::read("drive.bin").expect("Unable to read file");

        let data = fs::read("os.bin").expect("Unable to read file");
        let mut i = 0;
        while i < data.len() {
            RAM[i + 256 * 256] = data[i];
            i += 1;
        }

        loop {
            RAM[65527] = get_key();

            RAM[65528] = ((mouse_position().0 as u16) >> 8) as u8;
            RAM[65529] = (mouse_position().0 as u16) as u8;

            RAM[65530] = ((mouse_position().1 as u16) >> 8) as u8;
            RAM[65531] = (mouse_position().1 as u16) as u8;

            RAM[65532] = ((screen_width() as u16) >> 8) as u8;
            RAM[65533] = (screen_width() as u16) as u8;

            RAM[65534] = ((screen_height() as u16) >> 8) as u8;
            RAM[65535] = (screen_height() as u16) as u8;

            let instruction = RAM[cpu.mem_ptr];

            //println!("{:?}", instruction);

            //debug(cpu.clone(), functions.clone());

            match instruction {
                /*LDV*/0b0000_00000 => {
                    cpu.mem_ptr += 1;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = RAM[cpu.mem_ptr + 1];
                    cpu.mem_ptr += 1;
                }
                /*LDM*/0b0000_00001 => {
                    cpu.mem_ptr += 1;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = RAM[(((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16) as usize];
                    cpu.mem_ptr += 2;
                }
                /*STM*/0b0000_00010 => {
                    cpu.mem_ptr += 1;
                    RAM[(((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16) as usize] = cpu.r[RAM[cpu.mem_ptr] as usize];
                    cpu.mem_ptr += 2;
                }
                /*MOV*/0b0000_00011 => {
                    cpu.mem_ptr += 1;
                    cpu.r[RAM[cpu.mem_ptr + 1] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize];
                    cpu.mem_ptr += 1;
                }
                /*LDVZ*/0b0000_00100 => {
                    cpu.mem_ptr += 1;
                    if cpu.r[RAM[cpu.mem_ptr + 2] as usize] == 0 {
                        cpu.r[RAM[cpu.mem_ptr] as usize] = RAM[cpu.mem_ptr + 1];
                    }
                    cpu.mem_ptr += 2;
                }
                /*LVNZ*/0b0000_00101 => {
                    cpu.mem_ptr += 1;
                    if cpu.r[RAM[cpu.mem_ptr + 2] as usize] != 0 {
                        cpu.r[RAM[cpu.mem_ptr] as usize] = RAM[cpu.mem_ptr + 1];
                    }
                    cpu.mem_ptr += 2;
                }
                /*PUSH*/0b0000_00110 => {
                    cpu.mem_ptr += 1;
                    cpu.stack_ptr -= 1;
                    RAM[cpu.stack_ptr] = cpu.r[RAM[cpu.mem_ptr] as usize];
                }
                /*PSHV*/0b0000_00111 => {
                    cpu.mem_ptr += 1;
                    cpu.stack_ptr -= 1;
                    RAM[cpu.stack_ptr] = RAM[cpu.mem_ptr];
                }
                /*POP*/0b0000_01000 => {
                    cpu.mem_ptr += 1;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = RAM[cpu.stack_ptr];
                    cpu.stack_ptr += 1;
                }
                /*PPM*/0b0000_01001 => {
                    cpu.mem_ptr += 1;
                    RAM[((RAM[cpu.mem_ptr] as u16) << 8 | (RAM[cpu.mem_ptr + 1] as u16)) as usize] = RAM[cpu.stack_ptr];
                    cpu.stack_ptr += 1;
                    cpu.mem_ptr += 1;
                }
                /*OSTM*/0b0000_1010 => {
                    cpu.mem_ptr += 1;
                    RAM[((((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16) + cpu.r[RAM[cpu.mem_ptr + 3] as usize] as u16)as usize] = cpu.r[RAM[cpu.mem_ptr] as usize];
                    cpu.mem_ptr += 3;
                }
                /*OLDM*/0b0000_1011 => {
                    cpu.mem_ptr += 1;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = RAM[((((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16) + cpu.r[RAM[cpu.mem_ptr + 3] as usize] as u16) as usize];
                    cpu.mem_ptr += 3;
                }
                /*STVM*/0b0000_1100 => {
                    cpu.mem_ptr += 1;
                    RAM[(((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16) as usize] = RAM[cpu.mem_ptr];
                    cpu.mem_ptr += 2;
                }

                /*ADD*/0b0001_0000 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as u16
                        + cpu.r[RAM[cpu.mem_ptr + 1] as usize] as u16;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = val as u8;
                    if val > 255 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 1;
                }
                /*ADT*/0b0001_0001 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as u16
                        + cpu.r[RAM[cpu.mem_ptr + 1] as usize] as u16;
                    cpu.r[RAM[cpu.mem_ptr + 2] as usize] = val as u8;
                    if val > 255 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 2;
                }
                /*SUB*/0b0001_0010 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as i32 - cpu.r[RAM[cpu.mem_ptr + 1] as usize] as i32;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize].wrapping_sub(cpu.r[RAM[cpu.mem_ptr + 1] as usize]);
                    if val < 0 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 1;
                }
                /*SBT*/0b0001_0011 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as i32 - cpu.r[RAM[cpu.mem_ptr + 1] as usize] as i32;
                    cpu.r[RAM[cpu.mem_ptr + 2] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize].wrapping_sub(cpu.r[RAM[cpu.mem_ptr + 1] as usize]);
                    if val < 0 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 2;
                }
                /*ADV*/0b0001_0100 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as u16 + RAM[cpu.mem_ptr + 1] as u16;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = val as u8;
                    if val > 255 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 1;
                }
                /*ADVT*/0b0001_0101 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as u16 + RAM[cpu.mem_ptr + 1] as u16;
                    cpu.r[RAM[cpu.mem_ptr + 2] as usize] = val as u8;
                    if val > 255 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 2;
                }
                /*SBV*/0b0001_0110 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as i32 - RAM[cpu.mem_ptr + 1] as i32;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize].wrapping_sub(RAM[cpu.mem_ptr + 1]);
                    if val < 0 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 1;
                }
                /*SBVT*/0b0001_0111 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as i32 - RAM[cpu.mem_ptr + 1] as i32;
                    cpu.r[RAM[cpu.mem_ptr + 2] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize].wrapping_sub(RAM[cpu.mem_ptr + 1]);
                    if val < 0 {
                        cpu.carry_flag = true;
                    }
                    cpu.mem_ptr += 2;
                }
                /*INC*/0b0001_1000 => {
                    cpu.mem_ptr += 1;
                    let val = (cpu.r[RAM[cpu.mem_ptr] as usize] as u16) + 1;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = val as u8;
                    if val > 255 {
                        cpu.carry_flag = true;
                    }
                }
                /*DEC*/0b0001_1001 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as i32 - 1;
                    cpu.r[RAM[cpu.mem_ptr] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize].wrapping_sub(1);
                    if val < 0 {
                        cpu.carry_flag = true;
                    }
                }
                /*ADC*/0b0001_1010 => {
                    cpu.mem_ptr += 1;
                    if cpu.carry_flag {
                        cpu.r[RAM[cpu.mem_ptr] as usize] += 1;
                    }
                }
                /*CCF*/0b0001_1011 => {
                    cpu.carry_flag = false;
                }
                /*SCF*/0b0001_1100 => {
                    cpu.carry_flag = true;
                }
                /*SBC*/0b0001_1101 => {
                    cpu.mem_ptr += 1;
                    if cpu.carry_flag {
                        cpu.r[RAM[cpu.mem_ptr] as usize] = cpu.r[RAM[cpu.mem_ptr] as usize].wrapping_sub(1);
                    }
                }

                /*JMP*/0b0011_0000 => {
                    cpu.mem_ptr += 1;
                    if (RAM[cpu.mem_ptr] as i8) < 0 {
                        cpu.mem_ptr -= (RAM[cpu.mem_ptr] as i8 * -1) as usize + 2;
                    } else {
                        cpu.mem_ptr += RAM[cpu.mem_ptr] as usize - 1;
                    }
                }
                /*JPZ*/0b0011_0001 => {
                    cpu.mem_ptr += 2;
                    if cpu.r[RAM[cpu.mem_ptr] as usize] == 0 {
                        if (RAM[cpu.mem_ptr - 1] as i8) < 0 {
                            cpu.mem_ptr -= (RAM[cpu.mem_ptr - 1] as i8 * -1) as usize + 2;
                        } else {
                            cpu.mem_ptr += RAM[cpu.mem_ptr - 1] as usize - 1;
                        }
                    }
                }
                /*JPNZ*/0b0011_0010 => {
                    cpu.mem_ptr += 2;
                    if cpu.r[RAM[cpu.mem_ptr] as usize] != 0 {
                        if (RAM[cpu.mem_ptr - 1] as i8) < 0 {
                            cpu.mem_ptr -= (RAM[cpu.mem_ptr - 1] as i8 * -1) as usize + 2;
                        } else {
                            cpu.mem_ptr += RAM[cpu.mem_ptr - 1] as usize - 1;
                        }
                    }
                }
                /*FNC*/0b0011_0011 => {
                    cpu.mem_ptr += 1;
                    functions.insert(
                        ((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16,
                        cpu.mem_ptr + 1,
                    );
                    while RAM[cpu.mem_ptr] != 0b0011_0100 {
                        cpu.mem_ptr += 1;
                    }
                }
                /*RET*/0b0011_0100 => {
                    cpu.mem_ptr = called_from.pop().unwrap()-1;
                }
                /*CAL*/0b0011_0101 => {
                    called_from.push(cpu.mem_ptr);
                    cpu.mem_ptr += 1;
                    cpu.mem_ptr = functions.get(&(((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16)).unwrap().clone();
                }
                /*CLZ*/0b0011_0110 => {
                    if cpu.r[RAM[cpu.mem_ptr + 3] as usize] == 0 {
                        called_from.push(cpu.mem_ptr);
                        cpu.mem_ptr += 1;
                        cpu.mem_ptr = functions
                            .get(&(((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16))
                            .unwrap()
                            .clone();
                    } else {
                        cpu.mem_ptr += 3;
                    }
                }
                /*CLNZ*/0b0011_0111 => {
                    if cpu.r[RAM[cpu.mem_ptr + 3] as usize] != 0 {
                        called_from.push(cpu.mem_ptr);
                        cpu.mem_ptr += 1;
                        cpu.mem_ptr = functions
                            .get(&(((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16))
                            .unwrap()
                            .clone();
                    } else {
                        cpu.mem_ptr += 3;
                    }
                }

                /*MLT*/0b0100_0000 => {
                    cpu.mem_ptr += 1;
                    let val = cpu.r[RAM[cpu.mem_ptr] as usize] as u16 * cpu.r[RAM[cpu.mem_ptr + 1] as usize] as u16;
                    cpu.r[RAM[cpu.mem_ptr + 2] as usize] = (val >> 8) as u8;
                    cpu.r[RAM[cpu.mem_ptr + 3] as usize] = val as u8;
                    cpu.mem_ptr += 3;
                }
                /*DIV*/0b0100_0001 => {
                    cpu.mem_ptr += 1;
                    let val = (((cpu.r[RAM[cpu.mem_ptr] as usize] as u16) << 8) | cpu.r[RAM[cpu.mem_ptr + 1] as usize] as u16) /
                    (((cpu.r[RAM[cpu.mem_ptr + 2] as usize] as u16) << 8) | cpu.r[RAM[cpu.mem_ptr + 3] as usize] as u16);

                    cpu.r[RAM[cpu.mem_ptr + 4] as usize] = (val >> 8) as u8;
                    cpu.r[RAM[cpu.mem_ptr + 5] as usize] = val as u8;

                    cpu.mem_ptr += 5;
                }

                /*STD*/0b0110_0000 => {
                    cpu.mem_ptr += 1;
                    DRIVE[(((RAM[cpu.mem_ptr] as u32) << 24)
                        | ((RAM[cpu.mem_ptr + 1] as u32) << 16)
                        | ((RAM[cpu.mem_ptr + 2] as u32) << 8)
                        | RAM[cpu.mem_ptr + 3] as u32) as usize] =
                        cpu.r[RAM[cpu.mem_ptr + 4] as usize];
                    cpu.mem_ptr += 4;
                }
                /*SVD*/0b0110_0001 => {
                    cpu.mem_ptr += 1;
                    DRIVE[(((RAM[cpu.mem_ptr] as u32) << 24)
                        | ((RAM[cpu.mem_ptr + 1] as u32) << 16)
                        | ((RAM[cpu.mem_ptr + 2] as u32) << 8)
                        | RAM[cpu.mem_ptr + 3] as u32) as usize] = RAM[cpu.mem_ptr + 4];
                    cpu.mem_ptr += 4;
                }
                /*LDTM*/0b0110_0010 => {
                    cpu.mem_ptr += 1;

                    RAM[(((RAM[cpu.mem_ptr + 4] as u32) << 24)
                        | ((RAM[cpu.mem_ptr + 5] as u32) << 8)
                        | RAM[cpu.mem_ptr + 6] as u32) as usize] =
                        DRIVE[(((RAM[cpu.mem_ptr] as u32) << 24)
                            | ((RAM[cpu.mem_ptr + 1] as u32) << 16)
                            | ((RAM[cpu.mem_ptr + 2] as u32) << 8)
                            | RAM[cpu.mem_ptr + 3] as u32) as usize];

                    cpu.mem_ptr += 6;
                }
                /*LDML*/0b0110_0011 => {
                    cpu.mem_ptr += 1;
                    let mut i = 0;
                    while i
                        < (((RAM[cpu.mem_ptr + 7] as u16) << 8) | RAM[cpu.mem_ptr + 8] as u16)
                            as usize
                    {
                        RAM[(((RAM[cpu.mem_ptr + 4] as u32) << 24)
                            | ((RAM[cpu.mem_ptr + 5] as u32) << 8)
                            | RAM[cpu.mem_ptr + 6] as u32) as usize
                            + i] = DRIVE[(((RAM[cpu.mem_ptr] as u32) << 24)
                            | ((RAM[cpu.mem_ptr + 1] as u32) << 16)
                            | ((RAM[cpu.mem_ptr + 2] as u32) << 8)
                            | RAM[cpu.mem_ptr + 3] as u32)
                            as usize
                            + i];

                        i += 1;
                    }
                    cpu.mem_ptr += 8;
                }
                /*SMDL*/0b0110_0100 => {
                    cpu.mem_ptr += 1;
                    let mut i = 0;
                    while i
                        < (((RAM[cpu.mem_ptr + 7] as u16) << 8) | RAM[cpu.mem_ptr + 8] as u16)
                            as usize
                    {
                        DRIVE[(((RAM[cpu.mem_ptr] as u32) << 24)
                            | ((RAM[cpu.mem_ptr + 1] as u32) << 16)
                            | ((RAM[cpu.mem_ptr + 2] as u32) << 8)
                            | RAM[cpu.mem_ptr + 3] as u32)
                            as usize
                            + i] = RAM[(((RAM[cpu.mem_ptr + 4] as u32) << 24)
                            | ((RAM[cpu.mem_ptr + 5] as u32) << 8)
                            | RAM[cpu.mem_ptr + 6] as u32)
                            as usize
                            + i];

                        i += 1;
                    }
                    cpu.mem_ptr += 8;
                }

                /*WRT*/0b1000_0000 => {
                    cpu.mem_ptr += 1;
                    let text = id_to_str(cpu.r[RAM[cpu.mem_ptr] as usize]);
                    let size = RAM[cpu.mem_ptr + 1];

                    draw_text_ex(
                        &text,
                        (((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as f32,
                        (((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as f32,
                        TextParams {
                            font,
                            font_size: size as u16,
                            color: Color::new(
                                RAM[cpu.mem_ptr + 6] as f32 / 255.,
                                RAM[cpu.mem_ptr + 7] as f32 / 255.,
                                RAM[cpu.mem_ptr + 8] as f32 / 255.,
                                RAM[cpu.mem_ptr + 9] as f32 / 255.,
                            ),
                            ..Default::default()
                        },
                    );
                    cpu.mem_ptr += 9;
                }
                /*WRTL*/0b1000_0001 => {
                    cpu.mem_ptr += 1;
                    let end = RAM[cpu.mem_ptr] as usize + cpu.mem_ptr;

                    let mut text = String::new();
                    while cpu.mem_ptr < end {
                        cpu.mem_ptr += 1;
                        text.push_str(&id_to_str(RAM[cpu.mem_ptr]));
                    }

                    let size = RAM[cpu.mem_ptr + 1];

                    draw_text_ex(
                        &text,
                        (((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as f32,
                        (((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as f32,
                        TextParams {
                            font,
                            font_size: size as u16,
                            color: Color::new(
                                RAM[cpu.mem_ptr + 6] as f32 / 255.,
                                RAM[cpu.mem_ptr + 7] as f32 / 255.,
                                RAM[cpu.mem_ptr + 8] as f32 / 255.,
                                RAM[cpu.mem_ptr + 9] as f32 / 255.,
                            ),
                            ..Default::default()
                        },
                    );
                    cpu.mem_ptr += 9;
                }
                /*DRWR*/0b1000_0010 => {
                    cpu.mem_ptr += 1;
                    let x = RAM[cpu.mem_ptr];
                    let y = RAM[cpu.mem_ptr + 1];

                    draw_rectangle(
                        (((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as f32,
                        (((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as f32,
                        x as f32,
                        y as f32,
                        Color::new(
                            RAM[cpu.mem_ptr + 6] as f32 / 255.,
                            RAM[cpu.mem_ptr + 7] as f32 / 255.,
                            RAM[cpu.mem_ptr + 8] as f32 / 255.,
                            RAM[cpu.mem_ptr + 9] as f32 / 255.,
                        ),
                    );
                    cpu.mem_ptr += 9;
                }
                /*DRWC*/0b1000_0011 => {
                    cpu.mem_ptr += 1;
                    let r = RAM[cpu.mem_ptr];

                    draw_circle(
                        (((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16) as f32,
                        (((RAM[cpu.mem_ptr + 3] as u16) << 8) | RAM[cpu.mem_ptr + 4] as u16) as f32,
                        r as f32,
                        Color::new(
                            RAM[cpu.mem_ptr + 5] as f32 / 255.,
                            RAM[cpu.mem_ptr + 6] as f32 / 255.,
                            RAM[cpu.mem_ptr + 7] as f32 / 255.,
                            RAM[cpu.mem_ptr + 8] as f32 / 255.,
                        ),
                    );
                    cpu.mem_ptr += 8;
                }
                /*CLS*/0b1000_0100 => {
                    next_frame().await;
                    clear_background(BLACK);
                }
                /*MWRT*/0b1000_0101 => {
                    cpu.mem_ptr += 1;
                    let text = id_to_str(cpu.r[RAM[cpu.mem_ptr] as usize]);
                    let size = RAM[cpu.mem_ptr + 1];

                    draw_text_ex(
                        &text,
                        ((((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16)) as f32 + 
                        
                        ((((RAM[(((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as usize] as u16) << 8)) | 
                        (RAM[(((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as usize + 1] as u16)) as f32,
                        
                        ((((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16)) as f32 + 
                        
                        ((((RAM[(((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16) as usize] as u16) << 8)) | 
                        (RAM[(((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16) as usize + 1] as u16)) as f32,
                        
                        TextParams {
                            font,
                            font_size: size as u16,
                            color: Color::new(
                                RAM[cpu.mem_ptr + 10] as f32 / 255.,
                                RAM[cpu.mem_ptr + 11] as f32 / 255.,
                                RAM[cpu.mem_ptr + 12] as f32 / 255.,
                                RAM[cpu.mem_ptr + 13] as f32 / 255.,
                            ),
                            ..Default::default()
                        },
                    );
                    cpu.mem_ptr += 13;
                }
                /*MWRTL*/0b1000_0110 => {
                    cpu.mem_ptr += 1;
                    let end = RAM[cpu.mem_ptr] as usize + cpu.mem_ptr;

                    let mut text = String::new();
                    while cpu.mem_ptr < end {
                        cpu.mem_ptr += 1;
                        text.push_str(&id_to_str(RAM[cpu.mem_ptr]));
                    }

                    let size = RAM[cpu.mem_ptr + 1];

                    draw_text_ex(
                        &text,
                        ((((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16)) as f32 + 
                        
                        ((((RAM[(((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as usize] as u16) << 8)) | 
                        (RAM[(((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as usize + 1] as u16)) as f32,
                        
                        ((((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16)) as f32 + 
                        
                        ((((RAM[(((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16) as usize] as u16) << 8)) | 
                        (RAM[(((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16) as usize + 1] as u16)) as f32,
                        
                        TextParams {
                            font,
                            font_size: size as u16,
                            color: Color::new(
                                RAM[cpu.mem_ptr + 10] as f32 / 255.,
                                RAM[cpu.mem_ptr + 11] as f32 / 255.,
                                RAM[cpu.mem_ptr + 12] as f32 / 255.,
                                RAM[cpu.mem_ptr + 13] as f32 / 255.,
                            ),
                            ..Default::default()
                        },
                    );
                    cpu.mem_ptr += 13;
                }
                /*MDRWR*/0b1000_0111 => {
                    cpu.mem_ptr += 1;

                    // RAM[arg0] | RAM[arg0 + 1]
                    let w = (((RAM[(((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16) as usize) + 1] as u16)) as f32;

                    // RAM[arg1] | RAM[arg1 + 1]
                    let h = (((RAM[(((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as usize) + 1] as u16)) as f32;

                    let x = ((((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16)) as f32 + 
                        
                    (((RAM[(((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16) as usize) + 1] as u16)) as f32;

                    let y = ((((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16)) as f32 + 
                        
                    (((RAM[(((RAM[cpu.mem_ptr + 10] as u16) << 8) | RAM[cpu.mem_ptr + 11] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 10] as u16) << 8) | RAM[cpu.mem_ptr + 11] as u16) as usize) + 1] as u16)) as f32;

                    draw_rectangle(
                        x,
                        y,
                        
                        w as f32,
                        h as f32,
                        Color::new(
                            RAM[cpu.mem_ptr + 12] as f32 / 255.,
                            RAM[cpu.mem_ptr + 13] as f32 / 255.,
                            RAM[cpu.mem_ptr + 14] as f32 / 255.,
                            RAM[cpu.mem_ptr + 15] as f32 / 255.,
                        ),
                    );
                    cpu.mem_ptr += 15;
                }
                /*MDRWC*/0b1000_1000 => {
                    cpu.mem_ptr += 1;
                    let r = RAM[cpu.mem_ptr];

                    draw_circle(
                        ((((RAM[cpu.mem_ptr + 1] as u16) << 8) | RAM[cpu.mem_ptr + 2] as u16)) as f32 + 
                        
                        ((((RAM[(((RAM[cpu.mem_ptr + 3] as u16) << 8) | RAM[cpu.mem_ptr + 4] as u16) as usize] as u16) << 8)) | 
                        (RAM[(((RAM[cpu.mem_ptr + 3] as u16) << 8) | RAM[cpu.mem_ptr + 4] as u16) as usize + 1] as u16)) as f32,
                        
                        ((((RAM[cpu.mem_ptr + 5] as u16) << 8) | RAM[cpu.mem_ptr + 6] as u16)) as f32 + 
                        
                        ((((RAM[(((RAM[cpu.mem_ptr + 7] as u16) << 8) | RAM[cpu.mem_ptr + 8] as u16) as usize] as u16) << 8)) | 
                        (RAM[(((RAM[cpu.mem_ptr + 7] as u16) << 8) | RAM[cpu.mem_ptr + 8] as u16) as usize + 1] as u16)) as f32,
                        
                        r as f32,
                        Color::new(
                            RAM[cpu.mem_ptr + 9] as f32 / 255.,
                            RAM[cpu.mem_ptr + 10] as f32 / 255.,
                            RAM[cpu.mem_ptr + 11] as f32 / 255.,
                            RAM[cpu.mem_ptr + 12] as f32 / 255.,
                        ),
                    );
                    cpu.mem_ptr += 12;
                }
                /*DRWL*/0b1000_1001 => {
                    cpu.mem_ptr += 1;

                    let x1 = (((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as f32;
                    let y1 = (((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16) as f32;
                    let x2 = (((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16) as f32;
                    let y2 = (((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16) as f32;

                    let thick = RAM[cpu.mem_ptr + 10] as f32;

                    draw_line(
                        x1, y1,
                        x2, y2,
                        thick,
                        Color::new(
                            RAM[cpu.mem_ptr + 11] as f32 / 255.,
                            RAM[cpu.mem_ptr + 12] as f32 / 255.,
                            RAM[cpu.mem_ptr + 13] as f32 / 255.,
                            RAM[cpu.mem_ptr + 14] as f32 / 255.,
                        ),
                    );
                    cpu.mem_ptr += 14;
                }
                /*MDRWL*/0b1000_1010 => {
                    cpu.mem_ptr += 1;

                    let x1 = ((((RAM[cpu.mem_ptr] as u16) << 8) | RAM[cpu.mem_ptr + 1] as u16)) as f32 + 
                        
                    (((RAM[(((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 2] as u16) << 8) | RAM[cpu.mem_ptr + 3] as u16) as usize) + 1] as u16)) as f32;
                    
                    let y1 = ((((RAM[cpu.mem_ptr + 4] as u16) << 8) | RAM[cpu.mem_ptr + 5] as u16)) as f32 + 
                        
                    (((RAM[(((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 6] as u16) << 8) | RAM[cpu.mem_ptr + 7] as u16) as usize) + 1] as u16)) as f32;
                    
                    let x2 = ((((RAM[cpu.mem_ptr + 8] as u16) << 8) | RAM[cpu.mem_ptr + 9] as u16)) as f32 + 
                        
                    (((RAM[(((RAM[cpu.mem_ptr + 10] as u16) << 8) | RAM[cpu.mem_ptr + 11] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 10] as u16) << 8) | RAM[cpu.mem_ptr + 11] as u16) as usize) + 1] as u16)) as f32;
                    
                    let y2 = ((((RAM[cpu.mem_ptr + 12] as u16) << 8) | RAM[cpu.mem_ptr + 13] as u16)) as f32 + 
                        
                    (((RAM[(((RAM[cpu.mem_ptr + 14] as u16) << 8) | RAM[cpu.mem_ptr + 15] as u16) as usize] as u16) << 8) | 
                    (RAM[((((RAM[cpu.mem_ptr + 14] as u16) << 8) | RAM[cpu.mem_ptr + 15] as u16) as usize) + 1] as u16)) as f32;

                    let thick = RAM[cpu.mem_ptr + 16] as f32;

                    draw_line(
                        x1, y1,
                        x2, y2,
                        thick,
                        Color::new(
                            RAM[cpu.mem_ptr + 17] as f32 / 255.,
                            RAM[cpu.mem_ptr + 18] as f32 / 255.,
                            RAM[cpu.mem_ptr + 19] as f32 / 255.,
                            RAM[cpu.mem_ptr + 20] as f32 / 255.,
                        ),
                    );
                    cpu.mem_ptr += 20;
                }

                /*EXT*/0b1111_1110 => {
                    cpu.mem_ptr += 1;
                    cpu.r[7] = RAM[cpu.mem_ptr];
                    cpu.mem_ptr = 256 * 256 - 1;
                }
                /*HLT*/0b1111_1111 => {
                    fs::write("drive.bin", DRIVE.clone()).expect("Unable to write file");
                }

                _ => {}
            }
            cpu.mem_ptr += 1;
        }
    }
}

async fn drive_saver() {
    loop {
        unsafe {fs::write("drive.bin", DRIVE.clone()).expect("Unable to write file")}
        thread::sleep(time::Duration::from_millis(5000));
    }
}

fn get_key() -> u8 {
    let right_alt = is_key_down(KeyCode::RightAlt);
    let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
    let key = get_last_key_pressed();
    if key != None {
        let key = key.unwrap();
        if shift {
            match key {
                KeyCode::A => {return char_to_id("A")}
                KeyCode::B => {return char_to_id("B")}
                KeyCode::C => {return char_to_id("C")}
                KeyCode::D => {return char_to_id("D")}
                KeyCode::E => {return char_to_id("E")}
                KeyCode::F => {return char_to_id("F")}
                KeyCode::G => {return char_to_id("G")}
                KeyCode::H => {return char_to_id("H")}
                KeyCode::I => {return char_to_id("I")}
                KeyCode::J => {return char_to_id("J")}
                KeyCode::K => {return char_to_id("K")}
                KeyCode::L => {return char_to_id("L")}
                KeyCode::M => {return char_to_id("M")}
                KeyCode::N => {return char_to_id("N")}
                KeyCode::O => {return char_to_id("O")}
                KeyCode::P => {return char_to_id("P")}
                KeyCode::Q => {return char_to_id("Q")}
                KeyCode::R => {return char_to_id("R")}
                KeyCode::S => {return char_to_id("S")}
                KeyCode::T => {return char_to_id("T")}
                KeyCode::U => {return char_to_id("U")}
                KeyCode::V => {return char_to_id("V")}
                KeyCode::W => {return char_to_id("W")}
                KeyCode::X => {return char_to_id("X")}
                KeyCode::Y => {return char_to_id("Y")}
                KeyCode::Z => {return char_to_id("Z")}
                KeyCode::RightBracket => {return char_to_id("(")}
                KeyCode::Backslash => {return char_to_id(")")}
                KeyCode::Kp0 => {return char_to_id("0")}
                KeyCode::Kp1 => {return char_to_id("1")}
                KeyCode::Kp2 => {return char_to_id("2")}
                KeyCode::Kp3 => {return char_to_id("3")}
                KeyCode::Kp4 => {return char_to_id("4")}
                KeyCode::Kp5 => {return char_to_id("5")}
                KeyCode::Kp6 => {return char_to_id("6")}
                KeyCode::Kp7 => {return char_to_id("7")}
                KeyCode::Kp8 => {return char_to_id("8")}
                KeyCode::Kp9 => {return char_to_id("9")}
                KeyCode::Key0 => {return char_to_id("0")}
                KeyCode::Key1 => {return char_to_id("1")}
                KeyCode::Key2 => {return char_to_id("2")}
                KeyCode::Key3 => {return char_to_id("3")}
                KeyCode::Key4 => {return char_to_id("4")}
                KeyCode::Key5 => {return char_to_id("5")}
                KeyCode::Key6 => {return char_to_id("6")}
                KeyCode::Key7 => {return char_to_id("7")}
                KeyCode::Key8 => {return char_to_id("8")}
                KeyCode::Key9 => {return char_to_id("9")}
                KeyCode::Slash => {return char_to_id("_")}
                KeyCode::KpAdd => {return char_to_id("+")}
                KeyCode::KpSubtract => {return char_to_id("-")}
                KeyCode::KpMultiply => {return char_to_id("*")}
                KeyCode::KpDivide => {return char_to_id("/")}
                KeyCode::KpDecimal => {return char_to_id(".")}
                KeyCode::Period => {return char_to_id(":")}
                KeyCode::Comma => {return char_to_id("?")}
                KeyCode::Semicolon => {return char_to_id("\"")}
                KeyCode::GraveAccent => {return char_to_id("°")}
                KeyCode::Minus => {return char_to_id("%")}
                KeyCode::Apostrophe => {return char_to_id("!")}
                _ => {return 0b1111_0000}
            }
        }
        else if right_alt {
            match key {
                KeyCode::A => {return char_to_id("a")}
                KeyCode::B => {return char_to_id("{")}
                KeyCode::C => {return char_to_id("&")}
                KeyCode::D => {return char_to_id("Đ")}
                KeyCode::E => {return char_to_id("€")}
                KeyCode::F => {return char_to_id("[")}
                KeyCode::G => {return char_to_id("]")}
                KeyCode::H => {return char_to_id("h")}
                KeyCode::I => {return char_to_id("i")}
                KeyCode::J => {return char_to_id("j")}
                KeyCode::K => {return char_to_id("ł")}
                KeyCode::L => {return char_to_id("Ł")}
                KeyCode::M => {return char_to_id("m")}
                KeyCode::N => {return char_to_id("}")}
                KeyCode::O => {return char_to_id("o")}
                KeyCode::P => {return char_to_id("'")}
                KeyCode::Q => {return char_to_id("\\")}
                KeyCode::R => {return char_to_id("r")}
                KeyCode::S => {return char_to_id("đ")}
                KeyCode::T => {return char_to_id("t")}
                KeyCode::U => {return char_to_id("u")}
                KeyCode::V => {return char_to_id("@")}
                KeyCode::W => {return char_to_id("|")}
                KeyCode::X => {return char_to_id("#")}
                KeyCode::Y => {return char_to_id(">")}
                KeyCode::Z => {return char_to_id("z")}
                KeyCode::RightBracket => {return char_to_id("(")}
                KeyCode::Backslash => {return char_to_id(")")}
                KeyCode::Kp0 => {return char_to_id("0")}
                KeyCode::Kp1 => {return char_to_id("1")}
                KeyCode::Kp2 => {return char_to_id("2")}
                KeyCode::Kp3 => {return char_to_id("3")}
                KeyCode::Kp4 => {return char_to_id("4")}
                KeyCode::Kp5 => {return char_to_id("5")}
                KeyCode::Kp6 => {return char_to_id("6")}
                KeyCode::Kp7 => {return char_to_id("7")}
                KeyCode::Kp8 => {return char_to_id("8")}
                KeyCode::Kp9 => {return char_to_id("9")}
                KeyCode::Key0 => {return char_to_id("˝")}
                KeyCode::Key1 => {return char_to_id("~")}
                KeyCode::Key2 => {return char_to_id("ˇ")}
                KeyCode::Key3 => {return char_to_id("^")}
                KeyCode::Key4 => {return char_to_id("˘")}
                KeyCode::Key5 => {return char_to_id("°")}
                KeyCode::Key6 => {return char_to_id("˛")}
                KeyCode::Key7 => {return char_to_id("`")}
                KeyCode::Key8 => {return char_to_id("˙")}
                KeyCode::Key9 => {return char_to_id("´")}
                KeyCode::Slash => {return char_to_id("*")}
                KeyCode::KpAdd => {return char_to_id("+")}
                KeyCode::KpSubtract => {return char_to_id("-")}
                KeyCode::KpMultiply => {return char_to_id("*")}
                KeyCode::KpDivide => {return char_to_id("/")}
                KeyCode::KpDecimal => {return char_to_id(".")}
                KeyCode::Period => {return char_to_id(">")}
                KeyCode::Comma => {return char_to_id("<")}
                KeyCode::Semicolon => {return char_to_id("$")}
                KeyCode::GraveAccent => {return char_to_id(";")}
                KeyCode::Minus => {return char_to_id("¨")}
                KeyCode::Apostrophe => {return char_to_id("ß")}
                _ => {return 0b1111_0000}
            }
        }
        else {
            match key {
                KeyCode::A => {return char_to_id("a")}
                KeyCode::B => {return char_to_id("b")}
                KeyCode::C => {return char_to_id("c")}
                KeyCode::D => {return char_to_id("d")}
                KeyCode::E => {return char_to_id("e")}
                KeyCode::F => {return char_to_id("f")}
                KeyCode::G => {return char_to_id("g")}
                KeyCode::H => {return char_to_id("h")}
                KeyCode::I => {return char_to_id("i")}
                KeyCode::J => {return char_to_id("j")}
                KeyCode::K => {return char_to_id("k")}
                KeyCode::L => {return char_to_id("l")}
                KeyCode::M => {return char_to_id("m")}
                KeyCode::N => {return char_to_id("n")}
                KeyCode::O => {return char_to_id("o")}
                KeyCode::P => {return char_to_id("p")}
                KeyCode::Q => {return char_to_id("q")}
                KeyCode::R => {return char_to_id("r")}
                KeyCode::S => {return char_to_id("s")}
                KeyCode::T => {return char_to_id("t")}
                KeyCode::U => {return char_to_id("u")}
                KeyCode::V => {return char_to_id("v")}
                KeyCode::W => {return char_to_id("w")}
                KeyCode::X => {return char_to_id("x")}
                KeyCode::Y => {return char_to_id("y")}
                KeyCode::Z => {return char_to_id("z")}
                KeyCode::RightBracket => {return char_to_id("(")}
                KeyCode::Backslash => {return char_to_id(")")}
                KeyCode::Kp0 => {return char_to_id("0")}
                KeyCode::Kp1 => {return char_to_id("1")}
                KeyCode::Kp2 => {return char_to_id("2")}
                KeyCode::Kp3 => {return char_to_id("3")}
                KeyCode::Kp4 => {return char_to_id("4")}
                KeyCode::Kp5 => {return char_to_id("5")}
                KeyCode::Kp6 => {return char_to_id("6")}
                KeyCode::Kp7 => {return char_to_id("7")}
                KeyCode::Kp8 => {return char_to_id("8")}
                KeyCode::Kp9 => {return char_to_id("9")}
                KeyCode::Key0 => {return char_to_id("0")}
                KeyCode::Key1 => {return char_to_id("1")}
                KeyCode::Key2 => {return char_to_id("2")}
                KeyCode::Key3 => {return char_to_id("3")}
                KeyCode::Key4 => {return char_to_id("4")}
                KeyCode::Key5 => {return char_to_id("5")}
                KeyCode::Key6 => {return char_to_id("6")}
                KeyCode::Key7 => {return char_to_id("7")}
                KeyCode::Key8 => {return char_to_id("8")}
                KeyCode::Key9 => {return char_to_id("9")}
                KeyCode::Slash => {return char_to_id("-")}
                KeyCode::KpAdd => {return char_to_id("+")}
                KeyCode::KpSubtract => {return char_to_id("-")}
                KeyCode::KpMultiply => {return char_to_id("*")}
                KeyCode::KpDivide => {return char_to_id("/")}
                KeyCode::KpDecimal => {return char_to_id(".")}
                KeyCode::Period => {return char_to_id(".")}
                KeyCode::Comma => {return char_to_id(",")}
                KeyCode::Semicolon => {return char_to_id("\"")}
                KeyCode::GraveAccent => {return char_to_id(";")}
                KeyCode::Minus => {return char_to_id("=")}
                KeyCode::Apostrophe => {return char_to_id("!")}
                KeyCode::Up => {return 0b1000_0000}
                KeyCode::Down => {return 0b1000_0001}
                KeyCode::Left => {return 0b1000_0010}
                KeyCode::Right => {return 0b1000_0011}
                _ => {return 0b1111_0000}
            }
        }
    }
    return 0b1111_0000;
}

fn id_to_str(id: u8) -> String {
    match id {
        0b0000_0000 => return String::from("A"),
        0b0000_0001 => return String::from("B"),
        0b0000_0010 => return String::from("C"),
        0b0000_0011 => return String::from("D"),
        0b0000_0100 => return String::from("E"),
        0b0000_0101 => return String::from("F"),
        0b0000_0110 => return String::from("G"),
        0b0000_0111 => return String::from("H"),
        0b0000_1000 => return String::from("I"),
        0b0000_1001 => return String::from("J"),
        0b0000_1010 => return String::from("K"),
        0b0000_1011 => return String::from("L"),
        0b0000_1100 => return String::from("M"),
        0b0000_1101 => return String::from("N"),
        0b0000_1110 => return String::from("O"),
        0b0000_1111 => return String::from("P"),

        0b0001_0000 => return String::from("Q"),
        0b0001_0001 => return String::from("R"),
        0b0001_0010 => return String::from("S"),
        0b0001_0011 => return String::from("T"),
        0b0001_0100 => return String::from("U"),
        0b0001_0101 => return String::from("V"),
        0b0001_0110 => return String::from("W"),
        0b0001_0111 => return String::from("X"),
        0b0001_1000 => return String::from("Y"),
        0b0001_1001 => return String::from("Z"),
        0b0001_1010 => return String::from("<"),
        0b0001_1011 => return String::from(">"),
        0b0001_1100 => return String::from("("),
        0b0001_1101 => return String::from(")"),
        0b0001_1110 => return String::from("["),
        0b0001_1111 => return String::from("]"),

        0b0010_0000 => return String::from("a"),
        0b0010_0001 => return String::from("b"),
        0b0010_0010 => return String::from("c"),
        0b0010_0011 => return String::from("d"),
        0b0010_0100 => return String::from("e"),
        0b0010_0101 => return String::from("f"),
        0b0010_0110 => return String::from("g"),
        0b0010_0111 => return String::from("h"),
        0b0010_1000 => return String::from("i"),
        0b0010_1001 => return String::from("j"),
        0b0010_1010 => return String::from("k"),
        0b0010_1011 => return String::from("l"),
        0b0010_1100 => return String::from("m"),
        0b0010_1101 => return String::from("n"),
        0b0010_1110 => return String::from("o"),
        0b0010_1111 => return String::from("p"),

        0b0011_0000 => return String::from("q"),
        0b0011_0001 => return String::from("r"),
        0b0011_0010 => return String::from("s"),
        0b0011_0011 => return String::from("t"),

        0b0011_0101 => return String::from("u"),
        0b0011_0110 => return String::from("v"),
        0b0011_0111 => return String::from("w"),
        0b0011_1000 => return String::from("x"),
        0b0011_1001 => return String::from("y"),
        0b0011_1010 => return String::from("z"),
        0b0011_1011 => return String::from("!"),
        0b0011_1100 => return String::from("?"),
        0b0011_1101 => return String::from("|"),
        0b0011_1110 => return String::from("&"),
        0b0011_1111 => return String::from("#"),

        0b0100_0000 => return String::from("0"),
        0b0100_0001 => return String::from("1"),
        0b0100_0010 => return String::from("2"),
        0b0100_0011 => return String::from("3"),
        0b0100_0100 => return String::from("4"),
        0b0100_0101 => return String::from("5"),
        0b0100_0110 => return String::from("6"),
        0b0100_0111 => return String::from("7"),
        0b0100_1000 => return String::from("8"),
        0b0100_1001 => return String::from("9"),
        0b0100_1010 => return String::from("/"),
        0b0100_1011 => return String::from("\\"),
        0b0100_1100 => return String::from("~"),
        0b0100_1101 => return String::from("_"),
        0b0100_1110 => return String::from("-"),
        0b0100_1111 => return String::from("+"),

        0b0101_0000 => return String::from("*"),
        0b0101_0001 => return String::from("."),
        0b0101_0010 => return String::from(","),
        0b0101_0011 => return String::from(";"),
        0b0101_0100 => return String::from("="),
        0b0101_0101 => return String::from("%"),
        0b0101_0110 => return String::from("{"),
        0b0101_0111 => return String::from("}"),
        0b0101_1000 => return String::from(":"),
        0b0101_1001 => return String::from("\""),
        0b0101_1010 => return String::from("'"),
        0b0101_1011 => return String::from(" "),
        0b0101_1100 => return String::from(""),
        0b0101_1101 => return String::from(""),
        0b0101_1110 => return String::from(""),
        0b0101_1111 => return String::from(""),

        _ => return String::from("▯"),
    }
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
