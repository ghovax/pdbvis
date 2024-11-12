# pdbvis

A fast and lightweight 3D protein structure viewer that loads and visualizes proteins directly from the Protein Data Bank (PDB).

## Features

- Direct loading from PDB database using protein IDs
- Interactive 3D visualization with:
  - Backbone view mode
  - Cartoon view mode
- Real-time view mode switching
- Coordinate axes display
- Camera controls:
  - Zoom with mouse wheel
  - Rotate by dragging
  - Pan with right-click drag

## Installation

Ensure you have Rust installed ([rustup.rs](https://rustup.rs/)), then:

```bash
cargo install pdbvis
pdbvis --help
```

## Dependencies

- [kiss3d](https://github.com/sebcrozet/kiss3d) - Keep It Simple, Stupid 3D graphics engine
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client for PDB downloads
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [serde](https://github.com/serde-rs/serde) - Data serialization

### Controls

- **Mouse Controls**:
  - Left click + drag: Rotate view
  - Right click + drag: Pan view
  - Mouse wheel: Zoom in/out
- **Keyboard Controls**:
  - `;` (semicolon): Toggle between backbone and cartoon view

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- [RCSB Protein Data Bank](https://www.rcsb.org/) for providing protein structure data
- The Rust community for excellent libraries and tools