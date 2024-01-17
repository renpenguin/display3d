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
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl FromStr for MultiShader {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(MultiShader::None),
            "invert" => Ok(MultiShader::Invert),
            s if s.starts_with("flat-") => {
                let colour: Colour = s.replace("flat-", "").parse()?;

                Ok(MultiShader::FlatColour(colour))
            }
            "solid" => Ok(MultiShader::Solid),
            _ => Err(String::from("Invalid shader name")),
        }
    }
}

impl CanShade for MultiShader {
    fn shade(&mut self, pixel: Pixel) -> Pixel {
        match self {
            MultiShader::None => pixel,
            MultiShader::Invert => {
                let modifier = match pixel.fill_char.modifier {
                    Modifier::Colour(colour) => {
                        Modifier::from_rgb(255 - colour.r, 255 - colour.g, 255 - colour.b)
                    }
                    _ => Modifier::from_rgb(0, 0, 0),
                };

                Pixel::new(pixel.pos, ColChar::new(pixel.fill_char.text_char, modifier))
            }
            MultiShader::FlatColour(colour) => Pixel::new(
                pixel.pos,
                ColChar::new(pixel.fill_char.text_char, Modifier::Colour(*colour)),
            ),
            MultiShader::Solid => {
                Pixel::new(pixel.pos, ColChar::SOLID.with_mod(pixel.fill_char.modifier))
            }
        }
    }
}
