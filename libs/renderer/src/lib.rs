use std::path::PathBuf;

use ab_glyph::{FontRef, PxScale};
use anyhow::{Context, Error};
use imageproc::{
    self,
    image::{self, io::Reader as ImageReader, ImageBuffer, Rgba},
};

pub fn render(
    cardname: String,
    template_path: PathBuf,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    let cloned_path = template_path.clone();
    let img = ImageReader::open(cloned_path)
        .with_context(|| format!("Could not read file `{}`", template_path.display()))?
        .decode()?;

    let font = FontRef::try_from_slice(include_bytes!("../assets/Beleren.ttf"))?;

    let result: ImageBuffer<Rgba<u8>, Vec<u8>> = imageproc::drawing::draw_text(
        &img,
        image::Rgba([0u8, 0u8, 0u8, 255u8]),
        55,
        55,
        PxScale::from(70.0),
        &font,
        cardname.as_str(),
    );

    Ok(result)
}
