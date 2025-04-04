[![Rust](https://github.com/mlund/voronota-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mlund/voronota-rs/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/voronota)](https://crates.io/crates/voronota)
[![docs.rs](https://img.shields.io/docsrs/voronota)](https://docs.rs/voronota/latest/voronota)

# Voronota-LT

[Voronota-LT](https://www.voronota.com/expansion_lt) (pronounced ‘voronota lite’) is an alternative version of Voronota for constructing tessellation-derived atomic contact areas and volumes. Voronota-LT was written from scratch and does not use any external code, even from the core Voronota. The primary motivation for creating Voronota-LT was drastically increasing the speed of computing tessellation-based atom-atom contact areas and atom solvent-accessible surface areas.

Like Voronota, Voronota-LT can compute contact areas derived from the additively weighted Voronoi tessellation, but the main increase in speed comes when utilizing a simpler, radical tessellation variant, also known as Laguerre-Laguerre tessellation or power diagram. This is the default tessellation variant in Voronota-LT. It considers radii of atoms together with the rolling probe radius to define radical planes as bisectors between atoms.

## Example

The following illustrates basic use and how to e.g. calculate the total
solvent accessible surface area:

~~~ rust
use voronota::{Ball, RadicalTessellation};
let balls = vec![
    Ball { x: 0.0, y: 0.0, z: 0.0, r: 2.0 },
    Ball { x: 1.0, y: 0.0, z: 0.0, r: 2.0 },
];
let tessellation = RadicalTessellation::from_balls(1.4, &balls, None, false);

assert_eq!(tessellation.balls.len(), 2);
assert_eq!(tessellation.contacts.len(), 1);
assert_eq!(tessellation.cells.len(), 2);

let area = tessellation.cells.iter().map(|c| c.sas_area).sum::<f64>();
assert_eq!(area, 166.6300743464026);
~~~

## Current Status

- [x] Calculates contacts, surface areas and volumes.
- [x] Support for periodic boundary conditions.
- [x] Unit tests and examples.
- [x] Bindings via the [`cxx` crate](https://crates.io/crates/cxx).
- [ ] Partial update of positions.

