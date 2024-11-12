use anyhow::Result;
use kiss3d::nalgebra::Point3;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
pub struct Atom {
    pub position: Point3<f32>,
    pub atom_type: String,
    pub residue: String,
    pub chain_id: char,
}

pub fn download_pdb(id: &str) -> Result<String> {
    let url = format!("https://files.rcsb.org/download/{}.pdb", id);
    let response = reqwest::blocking::get(&url)?;
    Ok(response.text()?)
}

pub fn parse_pdb_from_string(content: &str) -> Result<Vec<Atom>> {
    let mut atoms = Vec::new();

    for line in content.lines() {
        if line.starts_with("ATOM") {
            let x = line[30..38].trim().parse::<f32>()?;
            let y = line[38..46].trim().parse::<f32>()?;
            let z = line[46..54].trim().parse::<f32>()?;
            let atom_type = line[12..16].trim().to_string();
            let residue = line[17..20].trim().to_string();
            let chain_id = line.chars().nth(21).unwrap_or('A');

            atoms.push(Atom {
                position: Point3::new(x, y, z),
                atom_type,
                residue,
                chain_id,
            });
        }
    }

    Ok(atoms)
} 