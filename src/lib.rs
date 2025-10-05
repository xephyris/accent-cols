use objc2_app_kit::NSColor;


#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b
        }
    }

    pub fn from_u32(r: u32, g: u32, b: u32) -> Self {
        Self {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    ColorSpaceConversionError,

}

pub fn get_accent_color() -> Result<Color, Error>{
    #[cfg(target_os="macos")] {
        let (mut red, mut green, mut blue) = (0.0, 0.0, 0.0);
        let rgb_color ;
        unsafe {
            use objc2_app_kit::NSColorSpace;
            let accent_colors = &*NSColor::controlAccentColor().clone();
            rgb_color = accent_colors.colorUsingColorSpace(&*NSColorSpace::deviceRGBColorSpace());
        }
        match rgb_color {
            Some(rgb_col) => {
                unsafe {
                    rgb_col.getRed_green_blue_alpha(&mut red as *mut _, &mut green as *mut _, &mut blue as *mut _, std::ptr::null_mut());
                }
                Ok(Color {
                    r: (red * 255.0 as f64).round() as u8,
                    g: (green * 255.0 as f64).round() as u8,
                    b: (blue * 255.0 as f64).round() as u8,
                })
            },
            None => {
                Err(Error::ColorSpaceConversionError)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_rgb() {
        dbg!(get_accent_color());
        assert!(false);
    }
}
