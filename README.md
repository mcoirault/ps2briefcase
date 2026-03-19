# PS2Briefcase

PS2Briefcase is a sister project of the excellent [PS2Suitcase](https://github.com/ps2store/ps2suitcase).

PS2Briefcase provides a simple CLI tool to create, read, and edit the `icon.sys` file used by the PS2 saves.

# Usage

PS2Briefcase uses subcommands, similar to a tool like `git`.  
By default, any subcommand will print the content of the .sys file. You can pass `--machine-readable` to any subcommand to get an output that is easier to parse, useful for scripting.  
`--help` can be used with any subcommand to get detailed help.

## Create

`ps2briefcase create [OPTIONS] <FILE>`

The subcommand creates a fresh file with some default values.  
You can pass options to set the file's value (see Options section).

Example:  
`ps2briefcase create --title1 "foobar" --ambient "#123ABC" icon.sys`

## Show

`ps2briefcase show [OPTIONS] <FILE>`

The subcommand reads and display the content of an existing file.

Example:  
`ps2briefcase show icon.sys`

## Edit

`ps2briefcase edit [OPTIONS] <FILE>`

The subcommand changes one or many values in an existing file.  
You can pass options to set the file's value (see Options section).

Example:  
`ps2briefcase edit --transparency 75 --light1-y 0.45 icon.sys`


## Options

The `create` and `edit` subcommands can take any of the following options in order to set values in the .sys file.

      --title1 <STRING>              First line of the title, maximum 15 characters
      --title2 <STRING>              Second line of the title, title1 and title2 must be less than 34 total characters
      --icon-list <STRING>           The filename of the default icon, usually list.icn
      --icon-copy <STRING>           The filename of the copy icon, usually copy.icn
      --icon-delete <STRING>         The filename of the delete icon, usually delete.icn
      --ambient-color <HEXCODE>      The color of the ambient light applied to the 3d model of the icon
      --transparency <INT>           Transparency of the background, between 0 and 100
      --background-color1 <HEXCODE>  Top left color of the background gradient
      --background-color2 <HEXCODE>  Top right color of the background gradient
      --background-color3 <HEXCODE>  Bottom left color of the background gradient
      --background-color4 <HEXCODE>  Bottom left color of the background gradient
      --light1-color <HEXCODE>       Color of the first source of light
      --light1-x <FLOAT>             X position of the first source of light, between 0 and 1
      --light1-y <FLOAT>             Y position of the first source of light, between 0 and 1
      --light1-z <FLOAT>             Z position of the first source of light, between 0 and 1
      --light2-color <HEXCODE>       Color of the second source of light
      --light2-x <FLOAT>             X position of the second source of light, between 0 and 1
      --light2-y <FLOAT>             Y position of the second source of light, between 0 and 1
      --light2-z <FLOAT>             Z position of the second source of light, between 0 and 1
      --light3-color <HEXCODE>       Color of the third source of light
      --light3-x <FLOAT>             X position of the third source of light, between 0 and 1
      --light3-y <FLOAT>             Y position of the third source of light, between 0 and 1
      --light3-z <FLOAT>             Z position of the third source of light, between 0 and 1

# Commpile

This project is written in Rust. To build your own binary, install Rust, and then run `cargo build` at the root of the project. Make sure you clone the repo with the submodules.

Alternatively, pre-compiled binaries for Ubuntu, MacOS, and Windows are available in the [Releases](https://github.com/mcoirault/ps2briefcase/releases).
