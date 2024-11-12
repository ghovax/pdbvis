use anyhow::Result;
use kiss3d::nalgebra::{Point3, Vector3};
use crate::pdb::{download_pdb, parse_pdb_from_string, Atom};

pub struct Protein {
    pub atoms: Vec<Atom>,
    pub center: Point3<f32>,
    pub max_radius: f32,
}

impl Protein {
    pub fn from_pdb_id_in_memory(id: &str) -> Result<Self> {
        // Download PDB file directly into memory
        let pdb_data = download_pdb(id)?;
        
        // Parse PDB data from string
        let atoms = parse_pdb_from_string(&pdb_data)?;
        
        // Calculate center and radius
        let center = Self::calculate_center(&atoms);
        let max_radius = Self::calculate_max_distance(&atoms, center);
        
        Ok(Self {
            atoms,
            center,
            max_radius,
        })
    }

    fn calculate_center(atoms: &[Atom]) -> Point3<f32> {
        let ca_atoms: Vec<_> = atoms.iter()
            .filter(|atom| atom.atom_type == "CA")
            .collect();
        
        let sum = ca_atoms.iter().fold(Vector3::zeros(), |acc, atom| {
            acc + atom.position.coords
        });
        
        Point3::from(sum / ca_atoms.len() as f32)
    }

    fn calculate_max_distance(atoms: &[Atom], center: Point3<f32>) -> f32 {
        atoms.iter()
            .filter(|atom| atom.atom_type == "CA")
            .map(|atom| (atom.position - center).magnitude())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(50.0)
    }
} 