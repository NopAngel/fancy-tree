//! Module for color choices that control what colors actually get displayed.
use super::Color;
use clap::ValueEnum;
use mlua::{FromLua, Lua};
use owo_colors::{
    AnsiColors::{
        self, Black, Blue, BrightBlack, BrightBlue, BrightCyan, BrightGreen, BrightMagenta,
        BrightRed, BrightWhite, BrightYellow, Cyan, Green, Magenta, Red, White, Yellow,
    },
    DynColors, OwoColorize,
    Stream::Stdout,
};
use std::fmt::Display;
use std::io::{self, Write};

/// Supports users choosing the colors they would like to display.
#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum ColorChoice {
    /// Let the application decide.
    ///
    /// *This checks if the `Stdout` stream supports colors.*
    Auto,
    /// Show all colors.
    On,
    /// Show only the 16 ANSI colors.
    Ansi,
    /// Don't show any colors.
    Off,
}

impl ColorChoice {
    /// Should colors support be automatically detected?
    #[inline]
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }

    /// Should colors be on/enabled?
    #[inline]
    pub fn is_on(&self) -> bool {
        matches!(self, Self::On)
    }

    /// Should colors be enabled but limited to the 16 ANSI colors?
    #[inline]
    pub fn is_ansi(&self) -> bool {
        matches!(self, Self::Ansi)
    }

    /// Should colors be off/disabled?
    #[inline]
    pub fn is_off(&self) -> bool {
        matches!(self, Self::Off)
    }

    /// Writes a colorized display value to the writer.
    pub fn write_to<W, D>(
        &self,
        writer: &mut W,
        display: D,
        fg: Option<Color>,
        bg: Option<Color>,
    ) -> io::Result<()>
    where
        W: Write,
        D: Display + OwoColorize,
    {
        match (self, fg, bg) {
            // NOTE These variants must be on top
            (Self::Off, _, _) | (_, None, None) => Self::off_write_to(writer, display),
            (Self::Auto, fg, bg) => Self::auto_write_to(writer, display, fg, bg),
            (Self::On, fg, bg) => Self::on_write_to(writer, display, fg, bg),
            (Self::Ansi, fg, bg) => Self::ansi_write_to(writer, display, fg, bg),
        }
    }

    /// Writes the display with color support detected.
    fn auto_write_to<W, D, Fg, Bg>(
        writer: &mut W,
        display: D,
        fg: Option<Fg>,
        bg: Option<Bg>,
    ) -> io::Result<()>
    where
        W: Write,
        D: Display + OwoColorize,
        DynColors: From<Fg>,
        DynColors: From<Bg>,
    {
        let fg = fg.map(DynColors::from);
        let bg = bg.map(DynColors::from);

        // HACK This assumes that the writer is always Stdout, which might not be best
        //      if we ever support other writers (Stderr, file, etc.).
        match (fg, bg) {
            (None, None) => unreachable!("Should use the off writer"),
            (Some(fg), None) => write!(
                writer,
                "{}",
                display.if_supports_color(Stdout, |display| display.color(fg))
            ),
            (None, Some(bg)) => write!(
                writer,
                "{}",
                display.if_supports_color(Stdout, |display| display.on_color(bg))
            ),
            (Some(fg), Some(bg)) => write!(
                writer,
                "{}",
                display.if_supports_color(Stdout, |display| display.color(fg).on_color(bg))
            ),
        }
    }

    /// Writes the display with no colorization.
    #[inline]
    fn off_write_to<W, D>(writer: &mut W, display: D) -> io::Result<()>
    where
        W: Write,
        D: Display,
    {
        write!(writer, "{display}")
    }

    /// Writes the display with colorization on.
    fn on_write_to<W, D, Fg, Bg>(
        writer: &mut W,
        display: D,
        fg: Option<Fg>,
        bg: Option<Bg>,
    ) -> io::Result<()>
    where
        W: Write,
        D: Display + OwoColorize,
        DynColors: From<Fg>,
        DynColors: From<Bg>,
    {
        let fg = fg.map(DynColors::from);
        let bg = bg.map(DynColors::from);
        match (fg, bg) {
            (None, None) => unreachable!("Should use the off writer"),
            (Some(fg), None) => write!(writer, "{}", display.color(fg)),
            (None, Some(bg)) => write!(writer, "{}", display.on_color(bg)),
            (Some(fg), Some(bg)) => write!(writer, "{}", display.color(fg).on_color(bg)),
        }
    }

    /// Writes the display with colorization set to ANSI.
    fn ansi_write_to<W, D>(
        writer: &mut W,
        display: D,
        fg: Option<Color>,
        bg: Option<Color>,
    ) -> io::Result<()>
    where
        W: Write,
        D: Display + OwoColorize,
    {
        let convert = |color: Color| DynColors::Ansi(Self::color_to_ansi(color));
        let fg = fg.map(convert);
        let bg = bg.map(convert);
        Self::on_write_to(writer, display, fg, bg)
    }

    /// Converts the [`Color`] to ANSI.
    fn color_to_ansi(color: Color) -> AnsiColors {
        match color {
            Color::Ansi(ansi) => ansi,
            Color::Rgb(r, g, b) => Self::ansi_from_rgb(r, g, b),
        }
    }

    /// Tries to get the closest ANSI color from RGB values.
    fn ansi_from_rgb(r: u8, g: u8, b: u8) -> AnsiColors {
        /// Stores colors to be indexed into by a 3-bit union of the RGB values.
        const COLOR_INDEX: [AnsiColors; 16] = [
            Black,
            Red,
            Green,
            Yellow,
            Blue,
            Magenta,
            Cyan,
            White,
            BrightBlack,
            BrightRed,
            BrightGreen,
            BrightYellow,
            BrightBlue,
            BrightMagenta,
            BrightCyan,
            BrightWhite,
        ];
        /// Converts a color channel into a single bit at the given index.
        #[inline]
        const fn channel_bit(channel: u8, index: u8) -> u8 {
            debug_assert!(index < 3);
            (channel >> 7) << index
        }

        let brightness_index: usize = if Self::rgb_is_bright(r, g, b) { 8 } else { 0 };
        let color_index = usize::from(channel_bit(r, 0) | channel_bit(g, 1) | channel_bit(b, 2));
        debug_assert!(color_index <= 0b111);
        let index = brightness_index + color_index;
        debug_assert!(index < COLOR_INDEX.len());
        COLOR_INDEX[index]
    }

    /// Detects if an RGB color is bright.
    const fn rgb_is_bright(r: u8, g: u8, b: u8) -> bool {
        /// Threshold between colors and their bright variants. If at least one color
        /// channel is above this value, we assume the color is bright.
        const BRIGHT_THRESHOLD: u8 = 0b1100_0000;

        (r | g | b) >= BRIGHT_THRESHOLD || Self::rgb_is_bright_black(r, g, b)
    }

    /// Detects if an RGB color is bright black, which is a special case since bright black
    /// is still dark compared to other colors.
    const fn rgb_is_bright_black(r: u8, g: u8, b: u8) -> bool {
        const BRIGHT_BLACK_MIN: u8 = 0b0100_0000;
        const BRIGHT_BLACK_MAX: u8 = 0b1000_0000;
        #[inline]
        const fn within_limits(channel: u8) -> bool {
            BRIGHT_BLACK_MIN <= channel && channel < BRIGHT_BLACK_MAX
        }

        within_limits(r) && within_limits(g) && within_limits(b)
    }
}

impl Default for ColorChoice {
    #[inline]
    /// The auto variant.
    fn default() -> Self {
        Self::Auto
    }
}

impl FromLua for ColorChoice {
    fn from_lua(value: mlua::Value, _lua: &Lua) -> mlua::Result<Self> {
        const VALID_VALUES: [&str; 4] = ["auto", "on", "off", "ansi"];
        let type_name = value.type_name();
        let make_conversion_error = || mlua::Error::FromLuaConversionError {
            from: type_name,
            to: String::from("ColorChoice"),
            message: Some(format!("Must be one of {VALID_VALUES:?} or nil")),
        };
        let color_choice = value
            .as_string()
            .ok_or_else(make_conversion_error)?
            .to_string_lossy();
        let color_choice = color_choice.as_str();
        let color_choice = match color_choice {
            "auto" => Self::Auto,
            "on" => Self::On,
            "off" => Self::Off,
            "ansi" => Self::Ansi,
            _ => return Err(make_conversion_error()),
        };
        Ok(color_choice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::bright_black(85, 85, 85, BrightBlack)]
    #[case::black(0, 0b10, 0b11, Black)]
    #[case::green(0, 0b1000_0000, 0b11, Green)]
    #[case::bright_green(0, 0b1100_0001, 0b11, BrightGreen)]
    #[case::bright_yellow(0b1000_0000, 0b1100_0001, 0b11, BrightYellow)]
    #[case::bright_white(0xFF, 0xFF, 0xFF, BrightWhite)]
    fn test_color_choice_ansi_from_rgb(
        #[case] r: u8,
        #[case] g: u8,
        #[case] b: u8,
        #[case] expected: AnsiColors,
    ) {
        assert_eq!(expected, ColorChoice::ansi_from_rgb(r, g, b));
    }
}
