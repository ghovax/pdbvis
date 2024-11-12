mod protein;
mod visualization;
mod pdb;

use anyhow::Result;
use kiss3d::window::Window;
use kiss3d::light::Light;
use clap::Parser;
use serde::Serialize;
use std::collections::HashSet;

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

#[derive(Serialize)]
struct ProteinInfo {
    id: String,
    atom_count: usize,
    center: Position,
    max_radius: f32,
    chains: Vec<char>,
}

#[derive(Serialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    let protein_id = args.pdb_id.to_uppercase();

    // Load protein
    let protein = Protein::from_pdb_id_in_memory(&protein_id)?;
    
    // Prepare protein information
    let chains: HashSet<_> = protein.atoms.iter()
        .map(|atom| atom.chain_id)
        .collect();
    
    let info = ProteinInfo {
        id: protein_id.clone(),
        atom_count: protein.atoms.len(),
        center: Position {
            x: protein.center.x,
            y: protein.center.y,
            z: protein.center.z,
        },
        max_radius: protein.max_radius,
        chains: chains.into_iter().collect(),
    };

    // Print JSON
    println!("{}", serde_json::to_string_pretty(&info)?);

    // Create window and scene
    let mut window = Window::new(&format!("Protein Viewer - {}", protein_id));
    window.set_light(Light::StickToCamera);
    
    // Create visualizer and render
    let mut visualizer = ProteinVisualizer::new(&mut window, &protein)?;
    visualizer.run()
}