use anyhow::Result;
use kiss3d::nalgebra::Point3;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Atom {
    pub position: Point3<f32>,
    pub atom_type: String,
    pub residue: String,
    pub chain_id: char,
    pub residue_number: i32,
    pub occupancy: f32,
    pub temperature_factor: f32,
}

#[derive(Deserialize, Serialize)]
pub struct PDBResponse {
    #[serde(rename = "struct")]
    pub structure: PDBStruct,
    #[serde(rename = "rcsb_entry_info")]
    pub entry_info: EntryInfo,
    #[serde(rename = "rcsb_primary_citation")]
    pub citation: Option<Citation>,
    pub exptl: Vec<Experimental>,
    #[serde(rename = "rcsb_entity_source_organism")]
    pub source_organism: Option<Vec<SourceOrganism>>,
    pub cell: Option<UnitCell>,
}

#[derive(Deserialize, Serialize)]
pub struct PDBStruct {
    pub title: String,
    pub pdbx_descriptor: Option<String>,
    pub entry_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct EntryInfo {
    pub molecular_weight: Option<f32>,
    pub deposited_atom_count: Option<i32>,
    pub deposited_modeled_polymer_monomer_count: Option<i32>,
    pub deposited_polymer_monomer_count: Option<i32>,
    pub polymer_entity_count_protein: Option<i32>,
    pub deposited_polymer_entity_instance_count: Option<i32>,
    pub deposition_date: Option<String>,
    pub release_date: Option<String>,
    pub structure_determination_methodology: Option<String>,
    pub experimental_method: Option<String>,
    pub resolution_combined: Option<Vec<f32>>,
}

#[derive(Deserialize, Serialize)]
pub struct Citation {
    #[serde(rename = "rcsb_authors")]
    pub authors: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Experimental {
    pub method: String,
    pub resolution: Option<f32>,
    pub r_value_working: Option<f32>,
    pub r_value_observed: Option<f32>,
}

#[derive(Deserialize, Serialize)]
pub struct UnitCell {
    pub length_a: f32,
    pub length_b: f32,
    pub length_c: f32,
    pub angle_alpha: f32,
    pub angle_beta: f32,
    pub angle_gamma: f32,
}

#[derive(Deserialize, Serialize)]
pub struct SourceOrganism {
    #[serde(rename = "rcsb_gene_name")]
    pub gene_name: Option<Vec<GeneName>>,
    pub scientific_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GeneName {
    pub value: String,
}

pub async fn fetch_pdb_info(pdb_id: &str) -> Result<PDBResponse> {
    let api_url = format!(
        "https://data.rcsb.org/rest/v1/core/entry/{}",
        pdb_id.to_uppercase()
    );

    let api_response = reqwest::get(&api_url).await?;
    let pdb_metadata = api_response.json::<PDBResponse>().await?;
    Ok(pdb_metadata)
}

pub fn parse_pdb_from_string(pdb_content: &str) -> Result<Vec<Atom>> {
    let mut atom_list = Vec::new();

    for pdb_line in pdb_content.lines() {
        if pdb_line.starts_with("ATOM") {
            let coordinate_x = pdb_line[30..38].trim().parse::<f32>()?;
            let coordinate_y = pdb_line[38..46].trim().parse::<f32>()?;
            let coordinate_z = pdb_line[46..54].trim().parse::<f32>()?;
            let atom_name = pdb_line[12..16].trim().to_string();
            let residue_name = pdb_line[17..20].trim().to_string();
            let chain_identifier = pdb_line.chars().nth(21).unwrap_or('A');
            let residue_number = pdb_line[22..26].trim().parse::<i32>().unwrap_or(0);
            let atom_occupancy = pdb_line[54..60].trim().parse::<f32>().unwrap_or(1.0);
            let temperature_factor = pdb_line[60..66].trim().parse::<f32>().unwrap_or(0.0);

            atom_list.push(Atom {
                position: Point3::new(coordinate_x, coordinate_y, coordinate_z),
                atom_type: atom_name,
                residue: residue_name,
                chain_id: chain_identifier,
                residue_number,
                occupancy: atom_occupancy,
                temperature_factor,
            });
        }
    }

    Ok(atom_list)
}

pub async fn download_pdb(pdb_id: &str) -> Result<String> {
    let download_url = format!("https://files.rcsb.org/download/{}.pdb", pdb_id);
    let pdb_response = reqwest::get(&download_url).await?;
    Ok(pdb_response.text().await?)
}
