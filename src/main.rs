mod protein;
mod visualization;
mod pdb;

use anyhow::Result;
use kiss3d::window::Window;
use kiss3d::light::Light;
use clap::Parser;

use crate::protein::Protein;
use crate::visualization::ProteinVisualizer;

/// A 3D protein structure viewer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PDB ID of the protein to visualize (e.g., 8KEX)
    #[arg(help = "PDB ID of the protein to visualize (e.g., 8KEX)")]
    pdb_id: String,

    /// Initial view mode (backbone or cartoon)
    #[arg(short, long, default_value = "backbone")]
    view: Option<String>,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    let protein_id = args.pdb_id.to_uppercase();

    // Create window and scene
    let mut window = Window::new(&format!("Protein Viewer - {}", protein_id));
    window.set_light(Light::StickToCamera);

    // Load protein
    let protein = Protein::from_pdb_id_in_memory(&protein_id)?;
    
    // Create visualizer and render
    let mut visualizer = ProteinVisualizer::new(&mut window, &protein)?;
    visualizer.run()
}