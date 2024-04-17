#[cfg(test)]
extern crate approx;

#[cxx::bridge]
mod ffi {
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
        /// List of registered balls (positions and radii)
        balls: Vec<Ball>,
        /// List of contacts between balls
        contacts: Vec<Contact>,
        /// List of cells (sas_area, volume, included)
        cells: Vec<Cell>,
    }

    unsafe extern "C++" {
        include!("voronota-rs/src/interface.h");
        fn from_balls(probe_radius: f64, balls: &Vec<Ball>) -> RadicalTessellation;
    }
}

impl RadicalTessellation {
    /// Construct tessellation from a list of balls and a probe radius.
    ///
    /// # Examples:
    /// ~~~
    /// use voronota_rs::{Ball, RadicalTessellation};
    /// let balls = vec![
    ///    Ball { x: 0.0, y: 0.0, z: 0.0, r: 2.0 },
    ///    Ball { x: 1.0, y: 0.0, z: 0.0, r: 2.0 },
    /// ];
    /// let tessellation = RadicalTessellation::from_balls(1.4, &balls);
    /// let total_area: f64 = tessellation.cells.iter().map(|c| c.sas_area).sum();
    ///
    /// assert_eq!(tessellation.balls.len(), 2);
    /// assert_eq!(tessellation.contacts.len(), 1);
    /// assert_eq!(tessellation.cells.len(), 2);
    /// assert_eq!(total_area, 166.6300743464026);
    /// ~~~
    pub fn from_balls(probe_radius: f64, balls: &Vec<Ball>) -> Self {
        ffi::from_balls(probe_radius, balls)
    }
}

pub use ffi::{Ball, Cell, Contact, RadicalTessellation};

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
        let tessellation = RadicalTessellation::from_balls(1.4, &balls);
        assert_eq!(tessellation.balls.len(), 2);
        assert_eq!(tessellation.contacts.len(), 1);
        assert_eq!(tessellation.cells.len(), 2);
        approx::assert_relative_eq!(tessellation.cells[0].sas_area, 83.31503717320129);
        approx::assert_relative_eq!(tessellation.cells[0].volume, 100.34561094831156);
        assert!(tessellation.cells[0].included);
        assert_eq!(tessellation.contacts[0].index_a, 0);
        assert_eq!(tessellation.contacts[0].index_b, 1);
        approx::assert_relative_eq!(tessellation.contacts[0].area, 35.53141291210056);
        approx::assert_relative_eq!(tessellation.contacts[0].arc_length, 21.130567978766745);
    }
}
