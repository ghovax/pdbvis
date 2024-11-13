mod protein;
mod visualization;
mod pdb;

use anyhow::Result;
use kiss3d::window::Window;
use kiss3d::light::Light;
use clap::Parser;
use tokio;

use crate::protein::Protein;
use crate::visualization::ProteinVisualizer;

/// A 3D protein structure viewer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PDB ID of the protein to visualize (e.g., 8KEX)
    #[arg(help = "PDB ID of the protein to visualize (e.g., 8KEX)")]
    pdb_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let command_line_args = Args::parse();
    let pdb_id = command_line_args.pdb_id.to_uppercase();

    // Fetch protein metadata and print it
    let protein_metadata = pdb::fetch_pdb_info(&pdb_id).await?;
    println!("{}", serde_json::to_string_pretty(&protein_metadata)?);
    
    // Load protein structure
    let protein_structure = Protein::from_pdb_id_in_memory(&pdb_id).await?;

    // Create visualization window
    let mut visualization_window = Window::new(&format!("Protein Viewer - {}", pdb_id));
    visualization_window.set_light(Light::StickToCamera);
    
    // Create visualizer and render
    let mut protein_visualizer = ProteinVisualizer::new(&mut visualization_window, &protein_structure)?;
    protein_visualizer.run()
}