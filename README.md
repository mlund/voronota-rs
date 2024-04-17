# Voronota-LT

Voronota-LT (pronounced ‘voronota lite’) is an alternative version of Voronota for constructing tessellation-derived atomic contact areas and volumes. Voronota-LT was written from scratch and does not use any external code, even from the core Voronota. The primary motivation for creating Voronota-LT was drastically increasing the speed of computing tessellation-based atom-atom contact areas and atom solvent-accessible surface areas.

Like Voronota, Voronota-LT can compute contact areas derived from the additively weighted Voronoi tessellation, but the main increase in speed comes when utilizing a simpler, radical tessellation variant, also known as Laguerre-Laguerre tessellation or power diagram. This is the default tessellation variant in Voronota-LT. It considers radii of atoms together with the rolling probe radius to define radical planes as bisectors between atoms.

For more information, see <https://www.voronota.com>

## Usage

This crate provides a simplified interface to Voronota-LT and can added to existing Cargo-based projects with:

~~~ console
cargo add voronota
~~~

The following illustrates basic use and how to e.g. extract the solvent accessible surface area:

~~~ rust
use voronota::{Ball, RadicalTessellation};
let balls = vec![
    Ball { x: 0.0, y: 0.0, z: 0.0, r: 2.0 },
    Ball { x: 1.0, y: 0.0, z: 0.0, r: 2.0 },
];
let tessellation = RadicalTessellation::from_balls(1.4, &balls);

assert_eq!(tessellation.balls.len(), 2);
assert_eq!(tessellation.contacts.len(), 1);
assert_eq!(tessellation.cells.len(), 2);

let total_area: f64 = tessellation.cells.iter().map(|c| c.sas_area).sum();
assert_eq!(total_area, 166.6300743464026);
~~~
