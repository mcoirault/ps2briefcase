mod args;
mod hex_colors;

use crate::args::{Commands, EditArgs};
use crate::hex_colors::HexColors;
use args::Cli;
use clap::Parser;
use ps2_filetypes::color::Color;
use ps2_filetypes::{ColorF, IconSys, Vector};
use std::process::exit;

const MAGIC_BYTES: &[u8; 4] = b"PS2D";

fn main() {
    let cli = Cli::parse();

    let mut icon_sys: IconSys;
    let machine_readable_output;

    match &cli.command {
        Commands::Create(args) => {
            if !args.clone().validate() {
                exit(2);
            }
            icon_sys = new_icon_sys();
            save_icon_sys(&args.file, &icon_sys);
            machine_readable_output = args.machine_readable;
        }
        Commands::Edit(args) => {
            if !args.clone().validate() {
                exit(2);
            }

            icon_sys = open_icon_sys(&args.file);
            apply_arguments(args, &mut icon_sys);
            save_icon_sys(&args.file, &icon_sys);
            machine_readable_output = args.machine_readable;
        }
        Commands::Show {
            file,
            machine_readable,
        } => {
            icon_sys = open_icon_sys(file);
            machine_readable_output = *machine_readable;
        }
    }

    if machine_readable_output {
        print_machine_readable_icon_sys(&icon_sys);
    } else {
        print_icon_sys(&icon_sys);
    }
}

fn apply_arguments(edit_args: &EditArgs, icon_sys: &mut IconSys) {
    let args = edit_args.clone();
    if let Some(title1) = args.title1 {
        icon_sys.title_line1 = title1;
        icon_sys.linebreak_pos = icon_sys.title_line1.len() as u8;
    }
    if let Some(title2) = args.title2 {
        icon_sys.title_line2 = title2;
        icon_sys.linebreak_pos = icon_sys.title_line1.len() as u8;
    }

    if let Some(icon_list) = args.icon_list {
        icon_sys.icon_file = icon_list;
    }
    if let Some(icon_copy) = args.icon_copy {
        icon_sys.icon_copy_file = icon_copy;
    }
    if let Some(icon_delete) = args.icon_delete {
        icon_sys.icon_delete_file = icon_delete;
    }

    if let Some(ambient_color) = args.ambient_color {
        icon_sys.ambient_color.update_from_hex(ambient_color);
    }

    if let Some(transparency) = args.transparency {
        icon_sys.background_transparency = transparency;
    }
    if let Some(background_color1) = args.background_color1 {
        icon_sys.background_colors[0].update_from_hex(background_color1);
    }
    if let Some(background_color2) = args.background_color2 {
        icon_sys.background_colors[1].update_from_hex(background_color2);
    }
    if let Some(background_color3) = args.background_color3 {
        icon_sys.background_colors[2].update_from_hex(background_color3);
    }
    if let Some(background_color4) = args.background_color4 {
        icon_sys.background_colors[3].update_from_hex(background_color4);
    }

    if let Some(light1_color) = args.light1_color {
        icon_sys.light_colors[0].update_from_hex(light1_color);
    }
    if let Some(light1_x) = args.light1_x {
        icon_sys.light_directions[0].x = light1_x;
    }
    if let Some(light1_y) = args.light1_y {
        icon_sys.light_directions[0].y = light1_y;
    }
    if let Some(light1_z) = args.light1_z {
        icon_sys.light_directions[0].z = light1_z;
    }

    if let Some(light2_color) = args.light2_color {
        icon_sys.light_colors[1].update_from_hex(light2_color);
    }
    if let Some(light2_x) = args.light2_x {
        icon_sys.light_directions[1].x = light2_x;
    }
    if let Some(light2_y) = args.light2_y {
        icon_sys.light_directions[1].y = light2_y;
    }
    if let Some(light2_z) = args.light2_z {
        icon_sys.light_directions[1].z = light2_z;
    }

    if let Some(light3_color) = args.light3_color {
        icon_sys.light_colors[2].update_from_hex(light3_color);
    }
    if let Some(light3_x) = args.light3_x {
        icon_sys.light_directions[2].x = light3_x;
    }
    if let Some(light3_y) = args.light3_y {
        icon_sys.light_directions[2].y = light3_y;
    }
    if let Some(light3_z) = args.light3_z {
        icon_sys.light_directions[2].z = light3_z;
    }
}

fn new_icon_sys() -> IconSys {
    let grey_colorf = ColorF {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    let grey_color = Color {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    let direction = Vector {
        x: 0.5,
        y: 0.5,
        z: 0.5,
        w: 0.0,
    };
    IconSys {
        flags: 0,
        linebreak_pos: 11,
        background_transparency: 25,
        background_colors: [grey_color, grey_color, grey_color, grey_color],
        light_directions: [direction, direction, direction],
        light_colors: [grey_colorf, grey_colorf, grey_colorf],
        ambient_color: grey_colorf,
        title_line1: "lorem ipsum".to_string(),
        title_line2: "dolor sit amet".to_string(),
        icon_file: "list.icn".to_string(),
        icon_copy_file: "copy.icn".to_string(),
        icon_delete_file: "del.icn".to_string(),
    }
}

fn open_icon_sys(file_path: &String) -> IconSys {
    let file_buffer = std::fs::read(file_path.clone()).unwrap_or_else(|_| {
        println!("ERROR: Unable to read file {:?}", file_path);
        exit(1)
    });

    // check that this is an actual .sys file related to the PS2
    if &file_buffer[0..4] != MAGIC_BYTES {
        println!("ERROR: {:?} is not a valid PS2 .sys file", file_path);
        exit(3)
    }

    IconSys::new(file_buffer)
}

fn print_icon_sys(sys: &IconSys) {
    println!("Title :");
    println!("{}", sys.title_line1);
    println!("{}", sys.title_line2);
    println!("Flags : {:#02X}", sys.flags);
    println!("-----");
    println!("Icon Files");
    println!("  List   : {}", sys.icon_file);
    println!("  Copy   : {}", sys.icon_copy_file);
    println!("  Delete : {}", sys.icon_delete_file);
    println!("-----");
    println!("Ambient Color  : {}", sys.ambient_color.to_hex());
    println!("Background");
    println!("  Transparency : {}%", sys.background_transparency);
    println!(
        "  Color        : {}     {}",
        sys.background_colors[0].to_hex(),
        sys.background_colors[1].to_hex()
    );
    println!(
        "                 {}     {}",
        sys.background_colors[2].to_hex(),
        sys.background_colors[3].to_hex()
    );
    println!("-----");
    for index in 0..3 {
        println!("Light {}", index + 1);
        println!("  Color : {}", sys.light_colors[index].to_hex());
        println!(
            "  XYZ   : {:.2} {:.2} {:.2}",
            sys.light_directions[index].x,
            sys.light_directions[index].y,
            sys.light_directions[index].z
        );
    }
}

fn print_machine_readable_icon_sys(sys: &IconSys) {
    println!("title1: {}", sys.title_line1);
    println!("title2: {}", sys.title_line2);
    println!("flags: {:#02X}", sys.flags);
    println!("icon_list: {}", sys.icon_file);
    println!("icon_copy: {}", sys.icon_copy_file);
    println!("icon_delete: {}", sys.icon_delete_file);
    println!("ambient: {}", sys.ambient_color.to_hex());
    println!("transparency: {}", sys.background_transparency);
    for index in 0..4 {
        println!(
            "background{}: {}",
            index + 1,
            sys.background_colors[index].to_hex(),
        );
    }
    for index in 0..3 {
        println!("light{}_color: {}", index, sys.light_colors[index].to_hex());
        println!("light{}_x: {}", index, sys.light_directions[index].x);
        println!("light{}_y: {}", index, sys.light_directions[index].y);
        println!("light{}_z: {}", index, sys.light_directions[index].z);
    }
}

fn save_icon_sys(file_path: &String, icon_sys: &IconSys) {
    std::fs::write(file_path.clone(), icon_sys.to_bytes().unwrap()).unwrap_or_else(|_| {
        println!("ERROR: Unable to save file {:?}", file_path);
        exit(4)
    });
}
