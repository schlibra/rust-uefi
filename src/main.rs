#![no_main]
#![no_std]

extern crate alloc;
mod sch;
// use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
#[allow(unused_imports)]
use log::info;
use sch::SCH;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::proto::console::pointer::Pointer;
// use uefi::proto::console::pointer::Pointer;
// use uefi::proto::console::text::Color;
use uefi::table::boot::ScopedProtocol;
use uefi::{prelude::*, Result};
// use uefi::proto::console::text::{Color, Output};

struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            pixels: vec![BltPixel::new(0, 0, 0); width * height],
        }
    }
    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }
    fn blit(&self, gop: &mut GraphicsOutput) -> Result {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }
}
#[allow(dead_code)]
static mut CURSER_POS:[i128; 2] = [0, 0];

// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8
// }

// impl Color {
//     fn new (red: u8, green: u8, blue: u8) -> Color {
//         Color::new(red, green, blue)
//     }
// }

fn abs_i128(num: i128) -> i128{
    if num > 0 {
        num
    } else {
        -num
    }
}

#[allow(dead_code)]
fn my(bt: &BootServices) -> Result {
    // let output_handle = bt.get_handle_for_protocol::<Output>().unwrap();
    // let mut output = bt.open_protocol_exclusive::<Output>(output_handle).unwrap();
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = bt
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();
    // let mut buf = [0; 400];
    // let mut message = CStr16::from_str_with_buf("Hello World", &mut buf).unwrap();
    // output.clear().unwrap();
    // output.enable_cursor(true).unwrap();
    // output.set_color(Color::Yellow, Color::Blue).unwrap();
    // output.output_string(&message).unwrap();
    // message = CStr16::from_str_with_buf("Welcome to my OS", &mut buf).unwrap();
    // output.set_cursor_position(0, 1).unwrap();
    // output.set_color(Color::Green, Color::Black).unwrap();
    // output.output_string(&message).unwrap();
    // output.set_color(Color::LightGray, Color::Black).unwrap();
    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);
    let mut black = false;

    let (min, max) = (50, 100);
    let mut x = min;
    #[allow(unused_assignments)]
    let mut color = min;
    loop {
        x += 1;
        if black {
            color = max + min - x;
        } else {
            color = x;
        }
        if x == max {
            x = min;
            black = !black;
        }
        for i in 0..width {
            for j in 0..height {
                let pixel = buffer.pixel(i, j).unwrap();
                pixel.red = color;
                pixel.green = color;
                pixel.blue = color;
            }
        }
        buffer.blit(&mut gop)?;
    }
}

fn draw_font(buffer: &mut Buffer, char: &str, m_x: usize, m_y: usize, color: (u8, u8, u8)) -> Result {
    let font = SCH::fonts_char(char);
    for x in 0..16 {
        for y in 0..16 {
            if font[y][x] == 1 {
                let pixel = buffer.pixel(x+m_x, y+m_y).unwrap();
                pixel.red = color.0;
                pixel.green = color.1;
                pixel.blue = color.2;
            }
        }
    }
    Ok(())
}

fn draw_line(buffer: &mut Buffer, pos1: (u32, u32), pos2: (u32, u32), color: (u8, u8, u8)) -> Result {
    let k: i32 = ((pos2.1-pos1.1)/(pos2.0-pos1.0)).try_into().unwrap();
    let b: i32 = pos1.1 as i32-k*pos1.0 as i32;
    for x in pos1.0..pos2.0 {
        let y = k * x as i32 + b;
        let pixel = buffer.pixel(x as usize, y as usize).unwrap();
        pixel.red = color.0;
        pixel.green = color.1;
        pixel.blue = color.2;
    }
    Ok(())
}

// #[allow(dead_code)]
fn draw_word(buffer: &mut Buffer, word: &str, p_x: usize, p_y: usize, color: (u8, u8, u8), size: usize, thin: usize) -> Result {
    let mut count = 0;
    for item in word.chars() {
        let font = SCH::fonts_char(String::from(item).as_str());
        for x in 0..16*size {
            for y in 0..16*size {
                if font[y/size][x/size] == 1 {
                    let pixel = buffer.pixel(p_x+(16-thin)*count*size+x, p_y+y).unwrap();
                    pixel.red = color.0;
                    pixel.green = color.1;
                    pixel.blue = color.2;
                }
            }
        }
        count += 1;
    }
    Ok(())
}

fn draw_rect(buffer: &mut Buffer, p_x: usize, p_y: usize, width: usize, height: usize, color: (u8, u8, u8)) -> Result {
    for x in 0..width {
        for y in 0..height {
            let pixel = buffer.pixel(x+p_x, y+p_y).unwrap();
            pixel.red=color.0;
            pixel.green=color.1;
            pixel.blue=color.2;
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn draw_window(buffer: &mut Buffer, title: &str, p_x: usize, p_y: usize, width: usize, height: usize, background: (u8, u8, u8), border: usize, border_color: (u8, u8, u8)) -> Result {
    draw_rect(buffer, p_x-border, p_y-border, width+border*2, height+border*2, border_color).unwrap();
    draw_rect(buffer, p_x, p_y, width, height, background).unwrap();
    // for x in 0..width+border*2 {
    //     for y in 0..height+border*2 {
    //         let pixel = buffer.pixel(x+p_x-border, y+p_y-border).unwrap();
    //         if background.0 > 30 {pixel.red = background.0 - 25;} else {pixel.red = background.0;}
    //         if background.1 > 30 {pixel.green = background.1 - 25;} else {pixel.green = background.1;}
    //         if background.2 > 30 {pixel.blue = background.2 - 25;} else {pixel.blue = background.2;}
    //     }
    // }
    // for x in 0..width {
    //     for y in 0..height {
    //         let pixel = buffer.pixel(x+p_x, y+p_y).unwrap();
    //         pixel.red = background.0;
    //         pixel.green = background.1;
    //         pixel.blue = background.2;
    //     }
    // }
    draw_word(buffer, title, p_x+5, p_y+5, (0, 0, 0), 1, 4).unwrap();
    draw_rect(buffer, p_x, p_y+25, width, 2, border_color).unwrap();
    draw_font(buffer, "min", p_x+width-68, p_y+5, (0, 0, 0)).unwrap();
    draw_font(buffer, "max", p_x+width-42, p_y+5, (0, 0, 0)).unwrap();
    draw_font(buffer, "close", p_x+width-20, p_y+5, (0, 0, 0)).unwrap();
    Ok(())
}

#[allow(unused_variables)]
fn draw_desktop(gop: &mut ScopedProtocol<GraphicsOutput>, bt: &BootServices, system_table: &SystemTable<Boot>) -> Result {
    let pointer_handle = bt.get_handle_for_protocol::<Pointer>().unwrap();
    let mut pointer = bt.open_protocol_exclusive::<Pointer>(pointer_handle).unwrap();
    pointer.reset(false).unwrap();
    let (width, height) = gop.current_mode_info().resolution();
    let mut buffer = Buffer::new(width, height);
    let dock_height = 50;
    // let pointer_handle = bt.get_handle_for_protocol::<Pointer>().unwrap();
    // let mut pointer = bt.open_protocol_exclusive::<Pointer>(pointer_handle).unwrap();
    // pointer.wait_for_input_event().unwrap();
    // info!("{:?}",pointer.mode().resolution);
    // info!("{:?}",pointer.mode().has_button);
    // info!("{:?}",pointer.read_state());
    // info!("{:?}",pointer.read_state().unwrap());
    // info!("{}, {}", width, height);
    loop {
        draw_rect(&mut buffer, 0, 0, width, height, (20, 50, 200)).unwrap();
        // info!("{}, {}", height-dock_height, height);
        draw_rect(&mut buffer, 0, height-dock_height, width, dock_height, (150, 150, 150)).unwrap();
        draw_rect(&mut buffer, 0, 0, width, 32, (120, 120, 175)).unwrap();
        // for x in 0..width {
        //     for y in 0..height {
        //         let pixel = buffer.pixel(x, y).unwrap();
        //         pixel.red = 20;
        //         pixel.blue = 200;
        //         pixel.green = 50;
        //     }
        // }
        // for x in 0..width{
        //     for y in height-dock_height..height{
        //         let pixel = buffer.pixel(x, y).unwrap();
        //         pixel.red = 150;
        //         pixel.blue = 150;
        //         pixel.green = 150;
        //     }
        // }
        // for x in 0..width {
        //     for y in 0..32 {
        //         let pixel = buffer.pixel(x, y).unwrap();
        //         pixel.red = 120;
        //         pixel.blue = 175;
        //         pixel.green = 120;
        //     }
        // }
        // let pointer_mode = pointer.mode();
        // #[allow(unused_variables)]
        // let position = pointer_mode.resolution;
        // info!("{:?}", position);
        // let pointer_state: Option<pointer::PointerState> = pointer.read_state().unwrap();
        // let movement = pointer_state.unwrap().relative_movement;
        let p_x: usize = unsafe { abs_i128(CURSER_POS[0]) } as usize;
        let p_y: usize = unsafe { abs_i128(CURSER_POS[1]) } as usize;
        // info!("{}, {}",p_x, p_y);
        
        // let cursor = sch::Icons::cursor();
        // for x in 0..16 {
        //     for y in 0..16 {
        //         if cursor[x][y] == 1 {
        //             let pixel = buffer.pixel(y+p_x, x+p_y).unwrap();
        //             pixel.red = 255;
        //             pixel.green = 255;
        //             pixel.blue = 255;
        //         }
        //     }
        // }
        // draw time
        let time = system_table.runtime_services().get_time().unwrap();
        // let mut num = SCH::fonts_number(3);
        let offset_hour = 0;
        let mut hour: u8 = time.hour() + offset_hour;
        if hour >= 24 {
            hour-=24;
        }
        let mut hour_str: String = hour.to_string();

        if hour_str.len() == 1 {
            hour_str = String::from("0") + &hour_str;
        }
        let mut minute_str: String = time.minute().to_string();
        if minute_str.len() == 1 {
            minute_str = String::from("0") + &minute_str;
        }
        let mut second_str: String = time.second().to_string();
        if second_str.len() == 1 {
            second_str = String::from("0") + &second_str;
        }
        let time_str =  hour_str + ":" + &minute_str + ":" + &second_str;
        draw_word(&mut buffer, &time_str.as_str(), width/2-64, 8, (255, 255, 255), 1, 4).unwrap();
        // let hour = time.hour()+8;
        // let hour0 = hour/10;
        // let hour1 = hour%10;
        // let minute = time.hour();
        // let minute0 = minute/10;
        // let minute1 = minute%10;
        // let second = time.second();
        // let second0 = second/10;
        // let second1 = second%10;
        // draw_font(&mut buffer, SCH::fonts_number(hour0), width/2-64, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_number(hour1), width/2-48, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_char("vs"), width/2-32, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_number(minute0), width/2-16, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_number(minute1), width/2, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_char("vs"), width/2+16, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_number(second0), width/2+32, 8, (255, 255, 255)).unwrap();
        // draw_font(&mut buffer, SCH::fonts_number(second1), width/2+46, 8, (255, 255, 255)).unwrap();
        // let mut word = "Hello";
        // word=&word[0..1];
        // draw_word(&mut buffer, "Hello", 0, 0, (255, 255, 255)).unwrap();
        draw_word(&mut buffer, "01234:56789", 20, 100, (255, 255, 255), 1, 4).unwrap();
        draw_word(&mut buffer, "Hello", 20, 116, (200, 200, 200), 4, 8).unwrap();
        // draw_word(&mut buffer, "你好", 20, 132, (190, 201, 23), 1).unwrap();
        // draw_word(&mut buffer, "你好", 20, 148, (190, 201, 23), 2).unwrap();
        // draw_word(&mut buffer, "你好", 20, 180, (190, 201, 23), 3).unwrap();
        draw_window(&mut buffer, "Form1", 320, 180, 450, 340, (200, 200, 200), 3, (150, 150, 150)).unwrap();
        let resolution = pointer.mode().resolution;
        let resolution_str: String= resolution[0].to_string() + "," + &resolution[1].to_string() + "," + &resolution[2].to_string();
        draw_word(&mut buffer, &resolution_str.as_str(), 330, 220, (0, 0, 0), 1, 0).unwrap();
        let state = pointer.read_state().unwrap();
        if state == None {
            draw_word(&mut buffer, "no", 330, 236, (0, 0, 0), 1, 0).unwrap();
        }else {
            let pointer_speed = 2;
            draw_word(&mut buffer, "yes", 330, 236, (0, 0, 0), 1, 0).unwrap();
            let movement = state.unwrap().relative_movement;
            let movement_str: String = movement[0].to_string() + "," + &movement[1].to_string() + "," + &movement[2].to_string();
            unsafe{
                CURSER_POS[0] += (movement[0] as i128) / (resolution[0] as i128) * pointer_speed;
                CURSER_POS[1] += (movement[1] as i128) / (resolution[1] as i128) * pointer_speed;
            }
            draw_word(&mut buffer, &movement_str.as_str(), 330, 252, (0, 0, 0), 1, 0).unwrap();
        }
        draw_word(&mut buffer, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", 330, 268, (0, 0, 0), 1, 0).unwrap();
        draw_word(&mut buffer, "abcdefghijklmnopqrstuvwxyz", 330, 284, (0, 0, 0), 1, 0).unwrap();
        draw_line(&mut buffer, (250, 350), (300, 550), (255, 255, 255)).unwrap();

        draw_font(&mut buffer, "cursor", p_x, p_y, (255,255,255)).unwrap();
        // for x in 0..16 {
        //     for y in 0..16 {
        //         if num[y][x] == 1 {
        //             let pixel = buffer.pixel(x+80, y+80).unwrap();
        //             pixel.red = 255;
        //             pixel.green = 255;
        //             pixel.blue = 255;
        //         }
        //     }
        // }
        buffer.blit(gop).unwrap();
        bt.stall(1000);
    };
}

fn splash(gop: &mut ScopedProtocol<GraphicsOutput>, bt: &BootServices, system_table: &SystemTable<Boot>) -> Result {
    let (width, height) = gop.current_mode_info().resolution();
    unsafe{
        CURSER_POS[0] = (width as i128) / 2;
        CURSER_POS[1] = (height as i128) / 2;
    }
    // info!("{}x{}", width, height);
    let mut buffer = Buffer::new(width, height);
    let (min, max, mut count) = (50, 100, 1);
    let mut i: u8 = min;
    let mut black = false;
    #[allow(unused_assignments)]
    let mut color = i;
    loop {
        i += 1;
        if black {
            color = max + min - i;
        } else {
            color = i;
        }
        if i == max {
            i = min;
            black = !black;
            count -= 1;
        } 
        draw_rect(&mut buffer, 0, 0, width, height, (color, color, color)).unwrap();
        // for x in 0..width {
        //     for y in 0..height {
        //         let pixel = buffer.pixel(x, y).unwrap();
        //         pixel.red = color;
        //         pixel.green = color;
        //         pixel.blue = color;
        //     }
        // }
        buffer.blit(gop)?;
        
        if count == 0 {
            return draw_desktop(gop, bt, &system_table);
        }
    }
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bt: &BootServices = system_table.boot_services();
    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = bt
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();
    // let pointer_handle = bt.get_handle_for_protocol::<Pointer>().unwrap();
    // #[allow(unused_mut, unused_variables)]
    // let mut pointer = bt.open_protocol_exclusive::<Pointer>(pointer_handle).unwrap();
    // pointer.reset(true).unwrap();
    // info!("{:?}", pointer.read_state());
    // loop {
        // #[allow(unused_variables)]
        // let pointer_state = pointer.read_state().unwrap().unwrap();
        // let movement = pointer_state.relative_movement;
        // info!("{}, {}", movement[0], movement[1]);
        // info!("111");
    // }
    splash(&mut gop, bt, &system_table).unwrap();
    // my(bt).unwrap();
    // system_table.boot_services().stall(10000000000);
    // #[allow(unreachable_code)]
    Status::SUCCESS
}
