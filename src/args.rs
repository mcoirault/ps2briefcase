use crate::hex_colors::HexCode;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "\
        A tool to read and edit icon.sys files used by the PS2's OSDMENU.\n\
        \n\
        Exit Codes:\n\
        0 -> no error\n\
        1 -> provided [OPTIONS] failed validation\n\
        2 -> <FILE PATH> invalid\n\
        3 -> provided <FILE PATH> is not a valid PS2 .sys file\n\
        \n\
        Example Use: ps2briefcase.exe --title1 \"foo bar\" --transparency 55 --ambient-color \"#123ABC\" --light3-z 0.23 icon.sys\
        "
)]
pub(crate) struct Args {
    #[arg(value_name = "FILE PATH", help = "The path to a .sys file")]
    pub(crate) file: String,

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
        help = "The color of the ambient light applied to the 3d model of the icon"
    )]
    pub(crate) ambient_color: Option<String>,

    #[arg(
        long,
        value_name = "INT",
        help = "Transparency of the background, between 0 and 100"
    )]
    pub(crate) transparency: Option<u8>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Top left color of the background gradient"
    )]
    pub(crate) background_color1: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Top right color of the background gradient"
    )]
    pub(crate) background_color2: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Bottom left color of the background gradient"
    )]
    pub(crate) background_color3: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Bottom left color of the background gradient"
    )]
    pub(crate) background_color4: Option<String>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Color of the first source of light"
    )]
    pub(crate) light1_color: Option<String>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "X position of the first source of light, between 0 and 1"
    )]
    pub(crate) light1_x: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Y position of the first source of light, between 0 and 1"
    )]
    pub(crate) light1_y: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Z position of the first source of light, between 0 and 1"
    )]
    pub(crate) light1_z: Option<f32>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Color of the second source of light"
    )]
    pub(crate) light2_color: Option<String>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "X position of the second source of light, between 0 and 1"
    )]
    pub(crate) light2_x: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Y position of the second source of light, between 0 and 1"
    )]
    pub(crate) light2_y: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Z position of the second source of light, between 0 and 1"
    )]
    pub(crate) light2_z: Option<f32>,

    #[arg(
        long,
        value_name = "HEXCODE",
        help = "Color of the third source of light"
    )]
    pub(crate) light3_color: Option<String>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "X position of the third source of light, between 0 and 1"
    )]
    pub(crate) light3_x: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Y position of the third source of light, between 0 and 1"
    )]
    pub(crate) light3_y: Option<f32>,

    #[arg(
        long,
        value_name = "FLOAT",
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

        if let Some(ambient_color) = self.ambient_color
            && !ambient_color.is_hexcode()
        {
            output = format!(
                "{}  --ambient-color must be hex code, {:?} was provided\n",
                output, ambient_color,
            );
            is_valid = false;
        }

        if let Some(transparency) = self.transparency
            && transparency >= 100
        {
            output = format!(
                "{}  --transparency must be between 0 and 100, {:?} was provided\n",
                output, transparency,
            );
            is_valid = false;
        }

        if let Some(background_color1) = self.background_color1
            && !background_color1.is_hexcode()
        {
            output = format!(
                "{}  --background-color1 must be hex code, {:?} was provided\n",
                output, background_color1,
            );
            is_valid = false;
        }

        if let Some(background_color2) = self.background_color2
            && !background_color2.is_hexcode()
        {
            output = format!(
                "{}  --background-color2 must be hex code, {:?} was provided\n",
                output, background_color2,
            );
            is_valid = false;
        }

        if let Some(background_color3) = self.background_color3
            && !background_color3.is_hexcode()
        {
            output = format!(
                "{}  --background-color3 must be hex code, {:?} was provided\n",
                output, background_color3,
            );
            is_valid = false;
        }

        if let Some(background_color4) = self.background_color4
            && !background_color4.is_hexcode()
        {
            output = format!(
                "{}  --background-color4 must be hex code, {:?} was provided\n",
                output, background_color4,
            );
            is_valid = false;
        }

        if let Some(light1_color) = self.light1_color
            && !light1_color.is_hexcode()
        {
            output = format!(
                "{}  --light1-color must be hex code, {:?} was provided\n",
                output, light1_color,
            );
            is_valid = false;
        }

        if let Some(light1_x) = self.light1_x
            && !is_coord_valid(light1_x)
        {
            output = format!(
                "{}  --light1-x must be between 0.0 and 1.0, {:?} was provided\n",
                output, light1_x,
            );
            is_valid = false;
        }

        if let Some(light1_y) = self.light1_y
            && !is_coord_valid(light1_y)
        {
            output = format!(
                "{}  --light1-y must be between 0.0 and 1.0, {:?} was provided\n",
                output, light1_y,
            );
            is_valid = false;
        }

        if let Some(light1_z) = self.light1_z
            && !is_coord_valid(light1_z)
        {
            output = format!(
                "{}  --light1- must be between 0.0 and 1.0, {:?} was provided\n",
                output, light1_z,
            );
            is_valid = false;
        }

        if let Some(light2_color) = self.light2_color
            && !light2_color.is_hexcode()
        {
            output = format!(
                "{}  --light2-color must be hex code, {:?} was provided\n",
                output, light2_color,
            );
            is_valid = false;
        }

        if let Some(light2_x) = self.light2_x
            && !is_coord_valid(light2_x)
        {
            output = format!(
                "{}  --light2-x must be between 0.0 and 1.0, {:?} was provided\n",
                output, light2_x,
            );
            is_valid = false;
        }

        if let Some(light2_y) = self.light2_y
            && !is_coord_valid(light2_y)
        {
            output = format!(
                "{}  --light2-y must be between 0.0 and 1.0, {:?} was provided\n",
                output, light2_y,
            );
            is_valid = false;
        }

        if let Some(light2_z) = self.light2_z
            && !is_coord_valid(light2_z)
        {
            output = format!(
                "{}  --light2-z must be between 0.0 and 1.0, {:?} was provided\n",
                output, light2_z,
            );
            is_valid = false;
        }

        if let Some(light3_color) = self.light3_color
            && !light3_color.is_hexcode()
        {
            output = format!(
                "{}  --light3-color must be a hex code, {:?} was provided\n",
                output, light3_color,
            );
            is_valid = false;
        }

        if let Some(light3_x) = self.light3_x
            && !is_coord_valid(light3_x)
        {
            output = format!(
                "{}  --light3-x must be between 0.0 and 1.0, {:?} was provided\n",
                output, light3_x,
            );
            is_valid = false;
        }

        if let Some(light3_y) = self.light3_y
            && !is_coord_valid(light3_y)
        {
            output = format!(
                "{}  --light3-y must be between 0.0 and 1.0, {:?} was provided\n",
                output, light3_y,
            );
            is_valid = false;
        }

        if let Some(light3_z) = self.light3_z
            && !is_coord_valid(light3_z)
        {
            output = format!(
                "{}  --light3-z must be between 0.0 and 1.0, {:?} was provided\n",
                output, light3_z,
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

fn is_coord_valid(coord: f32) -> bool {
    coord >= 0.0 && coord <= 1.0
}
