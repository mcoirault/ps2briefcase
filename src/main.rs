use std::process::exit;
use clap::Parser;
use ps2_filetypes::color::Color;
use ps2_filetypes::{ColorF, IconSys};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help="The path to a .sys file")]
    file: String,

    #[arg(long)]
    title: Option<String>,

    #[arg(long)]
    icon_list: Option<String>,

    #[arg(long)]
    icon_copy: Option<String>,

    #[arg(long)]
    icon_delete: Option<String>,

    #[arg(long)]
    ambient_color: Option<String>,

    #[arg(long, help="A number between 0 and 100")]
    transparency: Option<u8>,
}

const MAGIC_BYTES: &[u8; 4] = b"PS2D";

fn main() {
    let args = Args::parse();

    let file_buffer = std::fs::read(args.file.clone()).expect("File not found");

    // check that this is an actual .sys file related to the PS2
    if &file_buffer[0..4] != MAGIC_BYTES {
        println!("{} is not a valid .sys file. Exiting.", args.file);
        exit(1)
    }

    let mut icon_sys = IconSys::new(file_buffer.clone());

    apply_arguments(args, &mut icon_sys);

    print_icon_sys(icon_sys);
}

fn apply_arguments(args: Args, icon_sys: &mut IconSys) {
    if let Some(title) = args.title {
        icon_sys.title = title;
    }
}

fn print_icon_sys(sys: IconSys) {
    println!("Title: {}", sys.title);
    println!("Flags: {:#02X}", sys.flags);
    println!("-----");
    println!("Icon Files");
    println!("  List  : {}", sys.icon_file);
    println!("  Copy  : {}", sys.icon_copy_file);
    println!("  Delete: {}", sys.icon_delete_file);
    println!("-----");
    println!("Ambient Color : {}", colorf_to_hex(sys.ambient_color));
    println!("Background");
    println!("  Transparency: {}%", sys.background_transparency);
    println!("  Color       : {}     {}", color_to_hex(sys.background_colors[0]), color_to_hex(sys.background_colors[1]));
    println!("                {}     {}", color_to_hex(sys.background_colors[2]), color_to_hex(sys.background_colors[3]));
    println!("-----");
    for index in 0..3 {
        println!("Light {}", index + 1);
        println!("  Color: {}", colorf_to_hex(sys.light_colors[index]));
        println!("  XYZ  : {:.2} {:.2} {:.2}", sys.light_directions[index].x, sys.light_directions[index].y, sys.light_directions[index].z);
    }

}

fn colorf_to_hex(color: ColorF)-> String {
    format!("#{:02X}{:02X}{:02X}", (color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8)
}

fn color_to_hex(color: Color)-> String {
    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}
