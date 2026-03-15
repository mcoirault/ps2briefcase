mod args;
mod hex_colors;

use crate::hex_colors::HexColors;
use args::Args;
use clap::Parser;
use ps2_filetypes::IconSys;
use std::process::exit;

const MAGIC_BYTES: &[u8; 4] = b"PS2D";

fn main() {
    let args = Args::parse();

    if !args.clone().validate() {
        exit(1);
    }

    let file_buffer = std::fs::read(args.file.clone()).unwrap_or_else(|_| {
        println!("ERROR: Unable to read file {:?}", args.file);
        exit(2)
    });

    // check that this is an actual .sys file related to the PS2
    if &file_buffer[0..4] != MAGIC_BYTES {
        println!("ERROR: {} is not a valid PS2 .sys file", args.file);
        exit(3)
    }

    let mut icon_sys = IconSys::new(file_buffer.clone());

    apply_arguments(args.clone(), &mut icon_sys);

    print_icon_sys(icon_sys.clone());

    std::fs::write(args.file.clone(), icon_sys.to_bytes().unwrap()).unwrap_or_else(|_| {
        println!("ERROR: Unable to save file {:?}", args.file);
        exit(4)
    });
}

fn apply_arguments(args: Args, icon_sys: &mut IconSys) {
    if let Some(title1) = args.title1 {
        icon_sys.title_line1 = title1;
    }
    if let Some(title2) = args.title2 {
        icon_sys.title_line2 = title2;
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

fn print_icon_sys(sys: IconSys) {
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
