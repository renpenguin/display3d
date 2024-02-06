use std::{fmt::Display, str::FromStr};

use gemini_engine::elements::{
    containers::CanShade,
    view::{ColChar, Colour, Modifier},
    Pixel,
};

#[derive(Debug, Clone, Copy)]
pub enum MultiShader {
    None,
    Invert,
    FlatColour(Colour),
    Solid,
}

impl Display for MultiShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_lowercase())
    }
}

impl FromStr for MultiShader {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "invert" => Ok(Self::Invert),
            s if s.starts_with("flat-") => {
                let colour: Colour = s.replace("flat-", "").parse()?;

                Ok(Self::FlatColour(colour))
            }
            "solid" => Ok(Self::Solid),
            _ => Err(String::from("Invalid shader name")),
        }
    }
}

impl CanShade for MultiShader {
    fn shade(&mut self, pixel: Pixel) -> Pixel {
        match self {
            Self::None => pixel,
            Self::Invert => {
                let modifier = match pixel.fill_char.modifier {
                    Modifier::Colour(colour) => {
                        Modifier::from_rgb(255 - colour.r, 255 - colour.g, 255 - colour.b)
                    }
                    _ => Modifier::from_rgb(0, 0, 0),
                };

                Pixel::new(pixel.pos, ColChar::new(pixel.fill_char.text_char, modifier))
            }
            Self::FlatColour(colour) => Pixel::new(
                pixel.pos,
                ColChar::new(pixel.fill_char.text_char, Modifier::Colour(*colour)),
            ),
            Self::Solid => Pixel::new(pixel.pos, ColChar::SOLID.with_mod(pixel.fill_char.modifier)),
        }
    }
}
