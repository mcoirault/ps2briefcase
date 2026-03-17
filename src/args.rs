use crate::hex_colors::REGEX_PATTERN;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "\
        A tool to read and edit icon.sys files used by the PS2's OSDMENU.\n\
        \n\
        Exit Codes:\n\
        0 -> no error\n\
        1 -> <FILE> invalid\n\
        2 -> provided [OPTIONS] failed validation\n\
        3 -> provided <FILE> is not a valid PS2 .sys file\n\
        4 -> failed to save the <FILE>\n\
        \n\
        Example Use: ps2briefcase.exe --title1 \"foo bar\" --transparency 55 --ambient-color \"#123ABC\" --light3-z 0.23 icon.sys\
        ",
    arg_required_else_help(true)
)]
pub(crate) struct Args {
    #[arg(value_name = "FILE", help = "The path to a .sys file")]
    pub(crate) file: String,

    #[arg(long, help = "Returns a machine readable output")]
    pub(crate) machine_readable: bool,

    #[arg(
        long,
        value_name = "STRING",
        help = "First line of the title, maximum 15 characters"
    )]
    pub(crate) title1: Option<String>,

    #[arg(
        long,
        value_name = "STRING",
        help = "Second line of the title, title1 and title2 must be less than 34 total characters"
    )]
    pub(crate) title2: Option<String>,

    #[arg(
        long,
        value_name = "STRING",
        help = "The filename of the default icon, usually list.icn"
    )]
    pub(crate) icon_list: Option<String>,

    #[arg(
        long,
        value_name = "STRING",
        help = "The filename of the copy icon, usually copy.icn"
    )]
    pub(crate) icon_copy: Option<String>,

    #[arg(
        long,
        value_name = "STRING",
        help = "The filename of the delete icon, usually delete.icn"
    )]
    pub(crate) icon_delete: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "The color of the ambient light applied to the 3d model of the icon"
    )]
    pub(crate) ambient_color: Option<String>,

    #[arg(
        long,
        value_name = "INT",
        value_parser = is_valid_transparency,
        help = "Transparency of the background, between 0 and 100"
    )]
    pub(crate) transparency: Option<u32>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Top left color of the background gradient"
    )]
    pub(crate) background_color1: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Top right color of the background gradient"
    )]
    pub(crate) background_color2: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Bottom left color of the background gradient"
    )]
    pub(crate) background_color3: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Bottom left color of the background gradient"
    )]
    pub(crate) background_color4: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Color of the first source of light"
    )]
    pub(crate) light1_color: Option<String>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "X position of the first source of light, between 0 and 1"
    )]
    pub(crate) light1_x: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "Y position of the first source of light, between 0 and 1"
    )]
    pub(crate) light1_y: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "Z position of the first source of light, between 0 and 1"
    )]
    pub(crate) light1_z: Option<f32>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Color of the second source of light"
    )]
    pub(crate) light2_color: Option<String>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "X position of the second source of light, between 0 and 1"
    )]
    pub(crate) light2_x: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "Y position of the second source of light, between 0 and 1"
    )]
    pub(crate) light2_y: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "Z position of the second source of light, between 0 and 1"
    )]
    pub(crate) light2_z: Option<f32>,

    #[arg(
        long,
        value_name = "HEXCODE",
        value_parser = is_hexcode,
        help = "Color of the third source of light"
    )]
    pub(crate) light3_color: Option<String>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "X position of the third source of light, between 0 and 1"
    )]
    pub(crate) light3_x: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "Y position of the third source of light, between 0 and 1"
    )]
    pub(crate) light3_y: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        value_parser = is_coord,
        help = "Z position of the third source of light, between 0 and 1"
    )]
    pub(crate) light3_z: Option<f32>,
}

impl Args {
    pub fn validate(self) -> bool {
        let mut is_valid: bool = true;
        let mut output: String = "".to_string();

        if let Some(ref title1) = self.title1
            && title1.len() > 15
        {
            output = format!(
                "{}  --title1 must be less than 15 characters, {:?} was provided\n",
                output, title1,
            );
            is_valid = false;
        }

        if let Some(title1) = self.title1
            && let Some(title2) = self.title2
            && title1.len() + title2.len() > 34
        {
            output = format!(
                "{}  title1 and title2 combined must be less than 34 characters\n",
                output,
            );
            is_valid = false;
        }

        if !is_valid {
            println!("ERROR: there was an error in the parameters provided");
            print!("{}", output);
        }

        is_valid
    }
}

fn is_coord(s: &str) -> Result<f32, String> {
    let error_message = "a value between 0.0 and 1.0 must be provided";
    let coord: f32 = s.parse().map_err(|_| error_message)?;
    if coord >= 0.0 && coord <= 1.0 {
        Ok(coord)
    } else {
        Err(error_message.to_string())
    }
}

fn is_hexcode(s: &str) -> Result<String, String> {
    if Regex::new(REGEX_PATTERN).unwrap().is_match(s) {
        Ok(s.to_string())
    } else {
        Err("provide a color code like \"#123ABC\"".to_string())
    }
}

fn is_valid_transparency(s: &str) -> Result<u32, String> {
    let error_message = "a value between 0.0 and 1.0 must be provided";
    let transparency: u32 = s.parse().map_err(|_| error_message)?;
    if transparency >= 100 {
        Ok(transparency)
    } else {
        Err(error_message.to_string())
    }
}
