use gemini_engine::elements::{
    containers::CanShade,
    view::{ColChar, Colour, Modifier},
    Pixel,
};

pub enum MultiShader {
    None,
    Invert,
    FlatColour(Colour),
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
        }
    }
}
