use std::error::Error;
use clap::Parser;
use clio::{Input, Output};

use scan_color_fix::fix_color;

/// Corrects color scans made with Pantum M6500W scanner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input png file, use '-' for stdin
    #[clap(value_parser)]
    input: Input,
    
    /// Output png file, use '-' for stdout
    #[clap(value_parser)]
    output: Output,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    
    fix_color(&mut args.input, &mut args.output)?;
    
    return Ok(());
}
