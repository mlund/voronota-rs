//! # Voronota
//!
//! Voronota-LT (pronounced ‘voronota lite’) is an alternative version of Voronota for constructing tessellation-derived
//! atomic contact areas and volumes. Voronota-LT was written from scratch and does not use any external code,
//! even from the core Voronota. The primary motivation for creating Voronota-LT was drastically increasing the speed
//! of computing tessellation-based atom-atom contact areas and atom solvent-accessible surface areas.
//! Like Voronota, Voronota-LT can compute contact areas derived from the additively weighted Voronoi tessellation,
//! but the main increase in speed comes when utilizing a simpler, radical tessellation variant, also known as
//! Laguerre-Laguerre tessellation or power diagram. This is the default tessellation variant in Voronota-LT.
//! It considers radii of atoms together with the rolling probe radius to define radical planes as bisectors between atoms.
//!
//! # Example
//!
//! ~~~ rust
//! use voronota::{Ball, RadicalTessellation};
//! let balls = vec![
//!     Ball { x: 0.0, y: 0.0, z: 0.0, r: 2.0 },
//!     Ball { x: 1.0, y: 0.0, z: 0.0, r: 2.0 },
//! ];
//! let tessellation = RadicalTessellation::from_balls(1.4, &balls, None, false);
//!
//! assert_eq!(tessellation.balls.len(), 2);
//! assert_eq!(tessellation.contacts.len(), 1);
//! assert_eq!(tessellation.cells.len(), 2);
//!
//! let total_area: f64 = tessellation.cells.iter().map(|c| c.sas_area).sum();
//! assert_eq!(total_area, 166.6300743464026);
//! ~~~
//!

#[cfg(test)]
extern crate approx;

use std::f64::consts::PI;

#[cxx::bridge]
mod ffi {
    /// Simple point with x, y and z coordinates.
    #[derive(Debug, Default, PartialEq, Clone)]
    struct SimplePoint {
        x: f64,
        y: f64,
        z: f64,
    }
    /// Ball with a position (x, y, z) and a radius r.
    #[derive(Debug, Default, PartialEq, Clone)]
    struct Ball {
        /// x coordinate
        x: f64,
        /// y coordinate
        y: f64,
        /// z coordinate
        z: f64,
        /// radius
        r: f64,
    }
    /// Contact between two balls with indices index_a and index_b, area and arc_length.
    #[derive(Debug, Default, PartialEq, Clone)]
    struct Contact {
        /// First ball index
        index_a: i32,
        /// Second ball index
        index_b: i32,
        /// Contact area
        area: f64,
        /// Contact arc length
        arc_length: f64,
    }
    /// Cell with sas_area, volume and included flag.
    #[derive(Debug, Default, PartialEq, Clone)]
    struct Cell {
        /// Solvent-accessible surface area
        sas_area: f64,
        /// Cell volume
        volume: f64,
        /// Included flag
        included: bool,
    }

    /// Radical tessellation with a probe radius, list of balls, contacts and cells.
    #[derive(Debug, Default, PartialEq, Clone)]
    struct RadicalTessellation {
        /// Probe radius
        probe: f64,
        /// Periodic box corners
        periodic_box_corners: Vec<SimplePoint>,
        /// List of registered balls (positions and radii)
        balls: Vec<Ball>,
        /// List of contacts between balls
        contacts: Vec<Contact>,
        /// List of cells (sas_area, volume, included)
        cells: Vec<Cell>,
        /// Enable tessellation net (slower; default: false)
        pub tessellation_net: bool,
    }

    unsafe extern "C++" {
        include!("voronota/src/interface.h");
        fn from_balls(probe_radius: f64, balls: &Vec<Ball>, with_net: bool) -> RadicalTessellation;
        fn from_balls_pbc(
            probe_radius: f64,
            balls: &Vec<Ball>,
            periodic_box_corners: &Vec<SimplePoint>,
            with_net: bool,
        ) -> RadicalTessellation;
    }
}

impl RadicalTessellation {
    /// Construct tessellation from a list of balls and a probe radius.
    ///
    /// # Arguments:
    ///
    /// * `probe_radius` - Probe radius
    /// * `balls` - List of balls with position and radii information
    /// * `periodic_box_corners` - Optional cuboidal box corners if periodic boundary conditions
    ///
    /// # Examples:
    /// ~~~
    /// use voronota::{Ball, RadicalTessellation};
    /// let balls = vec![
    ///    Ball { x: 0.0, y: 0.0, z: 0.0, r: 2.0 },
    ///    Ball { x: 1.0, y: 0.0, z: 0.0, r: 2.0 },
    /// ];
    /// let tessellation = RadicalTessellation::from_balls(1.4, &balls, None, false);
    /// let total_area: f64 = tessellation.cells.iter().map(|c| c.sas_area).sum();
    ///
    /// assert_eq!(tessellation.balls.len(), 2);
    /// assert_eq!(tessellation.contacts.len(), 1);
    /// assert_eq!(tessellation.cells.len(), 2);
    /// assert_eq!(total_area, 166.6300743464026);
    /// ~~~
    pub fn from_balls(
        probe_radius: f64,
        balls: &Vec<Ball>,
        periodic_box_corners: Option<[SimplePoint; 2]>,
        with_net: bool,
    ) -> Self {
        match periodic_box_corners {
            Some(corners) => {
                assert!(corners[0].x < corners[1].x);
                assert!(corners[0].y < corners[1].y);
                assert!(corners[0].z < corners[1].z);
                ffi::from_balls_pbc(
                    probe_radius,
                    balls,
                    &vec![corners[0].clone(), corners[1].clone()],
                    with_net,
                )
            }
            None => ffi::from_balls(probe_radius, balls, false),
        }
    }

    /// Clear all balls, contacts and cells.
    pub fn clear(&mut self) {
        self.balls.clear();
        self.contacts.clear();
        self.cells.clear();
    }

    /// True if there are no balls.
    pub fn is_empty(&self) -> bool {
        self.balls.is_empty()
    }

    /// Total contact area of a single particle given by its `index`
    fn contact_area(&self, index: usize) -> f64 {
        self.contacts
            .iter()
            .filter(|&c| c.index_a == index as i32 || c.index_b == index as i32)
            .map(|contact| contact.area)
            .sum()
    }

    /// Available surface area of a single particle given by its `index`
    ///
    /// This is merely the total area of the particle minus the contact area.
    pub fn available_area(&self, index: usize) -> f64 {
        let radius = self.balls[index].r + self.probe;
        let total_area = 4.0 * PI * radius.powi(2);
        total_area - self.contact_area(index)
    }
}

pub use ffi::{Ball, Cell, Contact, RadicalTessellation, SimplePoint};

impl From<[f64; 3]> for SimplePoint {
    fn from(data: [f64; 3]) -> Self {
        SimplePoint {
            x: data[0],
            y: data[1],
            z: data[2],
        }
    }
}

impl From<[f64; 4]> for Ball {
    fn from(data: [f64; 4]) -> Self {
        Ball {
            x: data[0],
            y: data[1],
            z: data[2],
            r: data[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_balls() {
        let balls = vec![
            Ball {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                r: 2.0,
            },
            Ball {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                r: 2.0,
            },
        ];
        let tessellation = RadicalTessellation::from_balls(1.4, &balls, None, false);
        assert_eq!(tessellation.balls.len(), 2);
        assert_eq!(tessellation.contacts.len(), 1);
        assert_eq!(tessellation.cells.len(), 2);
        approx::assert_relative_eq!(
            tessellation.cells[0].sas_area,
            83.31503717320129,
            epsilon = 1e-6
        );
        approx::assert_relative_eq!(
            tessellation.cells[0].volume,
            100.34561094831156,
            epsilon = 1e-6
        );
        assert!(tessellation.cells[0].included);
        assert_eq!(tessellation.contacts[0].index_a, 0);
        assert_eq!(tessellation.contacts[0].index_b, 1);
        approx::assert_relative_eq!(
            tessellation.contacts[0].area,
            35.53141291210056,
            epsilon = 1e-6
        );
        approx::assert_relative_eq!(
            tessellation.contacts[0].arc_length,
            21.130567978766745,
            epsilon = 1e-6
        );

        approx::assert_relative_eq!(
            tessellation.available_area(0),
            109.73583138989146,
            epsilon = 1e-6
        );
        approx::assert_relative_eq!(
            tessellation.available_area(1),
            109.73583138989146,
            epsilon = 1e-6
        );
    }
}
