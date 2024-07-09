use clap::Parser;
use renderer::render;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of the card to render
    #[arg(short, long)]
    cardname: String,

    // Path to template image to use
    #[arg(short, long)]
    template_path: PathBuf,

    #[arg(short, long, default_value = "output.png")]
    output_path: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let result = render(args.cardname, args.template_path)?;

    let _ = result.save(args.output_path)?;

    Ok(())
}
