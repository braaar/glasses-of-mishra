use ab_glyph;
use ab_glyph::{FontRef, PxScale};
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use imageproc::image;
use imageproc::image::io::Reader as ImageReader;

#[derive(Parser)]
struct Args {
    /// The cardname to render on the template
    cardname: String,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let path = "templates/blue.png";

    let img = ImageReader::open(path)
        .with_context(|| format!("Could not read file `{}`", path))?
        .decode()?;

    let font = FontRef::try_from_slice(include_bytes!("../assets/Beleren.ttf"))?;

    let result = imageproc::drawing::draw_text(
        &img,
        image::Rgba([0u8, 0u8, 0u8, 255u8]),
        55,
        55,
        PxScale::from(70.0),
        &font,
        args.cardname.as_str(),
    );
    let _ = result.save("output.png");

    Ok(())
}
