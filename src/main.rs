use std::path::PathBuf;

use composition_analyzer::*;
use image;
use clap::Parser;


#[derive(Parser)]
#[command(name = "composition-analyzer")]
#[command(about = "Analisa composição visual de imagens", long_about = None)]
struct Args {
    #[arg(short, long)]
    input:  PathBuf,
    
    #[arg(short, long)]
    output: Option<PathBuf>,

}


fn main() {
    let args = Args::parse();
    println!("Carregando imagem: {:?}", args.input);

    let img = image::open(&args.input).expect("Failed to open image");

    let analysis = analyze_rule_of_thirds(&img);

    let output = draw_thirds_grid(&img, &analysis.intersection_points);
    
    if let Some(output_path) = args.output {
        output.save(output_path).expect("Failed to save image");
        println!("Image with grid saved successfully.");
    }

}
