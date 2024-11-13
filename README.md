# pdbvis

A high-performance 3D molecular visualization tool for rendering and analyzing protein structures from the Protein Data Bank (PDB).

## Overview

pdbvis provides real-time, interactive visualization of protein structures with multiple representation modes and intuitive camera controls. The application emphasizes efficiency and ease-of-use while maintaining scientific accuracy.

## Installation

```bash
cargo install pdbvis
```

## Usage

```bash
# Visualize a protein structure
pdbvis --pdb-id 8KEX

# Get help
pdbvis --help
```

## Controls

Use the `;` key to toggle between backbone and cartoon view modes. Drag with the mouse to rotate the view, right-click + drag to pan, and use the mouse wheel to zoom in and out.