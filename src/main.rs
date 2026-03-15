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

    apply_arguments(args, &mut icon_sys);

    print_icon_sys(icon_sys);
}

fn apply_arguments(args: Args, icon_sys: &mut IconSys) {
    if let Some(title1) = args.title1 {
        icon_sys.title_line1 = title1;
    }
    if let Some(title2) = args.title2 {
        icon_sys.title_line2 = title2;
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
