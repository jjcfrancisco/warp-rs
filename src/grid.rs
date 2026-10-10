use crate::affine::Affine;

/// The shape of a raster: how many pixels it has and where they sit on the ground.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub transform: Affine,
}

impl Grid {
    /// Reports whether a pixel position falls inside the grid. The far edges are outside.
    ///
    /// Source: <https://gdal.org/en/stable/user/raster_data_model.html#affine-geotransform>
    pub fn contains(&self, column: f64, row: f64) -> bool {
        column >= 0.0 && column < self.width as f64 && row >= 0.0 && row < self.height as f64
    }
    /// Returns the box the grid covers on the ground, as (left, bottom, right, top).
    ///
    /// Source: <https://github.com/rasterio/rasterio/blob/main/rasterio/_base.pyx>
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let width = self.width as f64;
        let height = self.height as f64;
        let corners = [
            self.transform.xy(0.0, 0.0),
            self.transform.xy(width, 0.0),
            self.transform.xy(0.0, height),
            self.transform.xy(width, height),
        ];
        let (mut left, mut bottom, mut right, mut top) = (
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        );
        for (x, y) in corners {
            left = left.min(x);
            bottom = bottom.min(y);
            right = right.max(x);
            top = top.max(y);
        }
        (left, bottom, right, top)
    }
    /// Maps a pixel of this grid to the matching pixel of source, if it lands inside.
    pub fn locate(&self, source: &Grid, column: f64, row: f64) -> Option<(f64, f64)> {
        let (source_column, source_row) =
            self.transform.locate_in(&source.transform, column, row)?;
        if source.contains(source_column, source_row) {
            Some((source_column, source_row))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn north_up() -> Grid {
        Grid {
            width: 10,
            height: 8,
            transform: Affine::from_gdal([100.0, 10.0, 0.0, 500.0, 0.0, -10.0]),
        }
    }

    #[test]
    fn contains_origin() {
        assert!(north_up().contains(0.0, 0.0));
    }

    #[test]
    fn contains_rejects_far_corner() {
        assert!(!north_up().contains(10.0, 8.0));
    }

    #[test]
    fn contains_centre_of_last_pixel() {
        assert!(north_up().contains(9.5, 7.5));
    }

    #[test]
    fn contains_rejects_negative_column() {
        assert!(!north_up().contains(-0.5, 0.0));
    }

    #[test]
    fn bounds_north_up() {
        assert_eq!(north_up().bounds(), (100.0, 420.0, 200.0, 500.0));
    }

    #[test]
    fn bounds_rotated_are_still_ordered() {
        // Turned 90 degrees: columns run north, rows run east.
        let rotated = Grid {
            transform: Affine::from_gdal([100.0, 0.0, 10.0, 500.0, 10.0, 0.0]),
            ..north_up()
        };
        let (left, bottom, right, top) = rotated.bounds();
        assert!(left < right);
        assert!(bottom < top);
    }

    #[test]
    fn bounds_rotated_30_degrees_uses_all_four_corners() {
        // Expected values come from the affine package and GDAL.
        let rotated = Grid {
            width: 12,
            height: 7,
            transform: Affine::from_gdal([
                1000.0,
                8.660254037844387,
                5.0,
                2000.0,
                5.0,
                -8.660254037844387,
            ]),
        };
        let (left, bottom, right, top) = rotated.bounds();
        assert_eq!(left, 1000.0);
        assert!((bottom - 1939.3782217350893).abs() < 1e-9);
        assert!((right - 1138.9230484541326).abs() < 1e-9);
        assert_eq!(top, 2060.0);
    }

    #[test]
    fn locate_same_grid_is_identity() {
        let grid = north_up();
        assert_eq!(grid.locate(&grid, 3.0, 2.0), Some((3.0, 2.0)));
    }

    #[test]
    fn locate_half_size_pixels_all_land_inside() {
        let source = north_up();
        let destination = Grid {
            width: 20,
            height: 16,
            transform: Affine::from_gdal([100.0, 5.0, 0.0, 500.0, 0.0, -5.0]),
        };
        for row in 0..destination.height {
            for column in 0..destination.width {
                assert!(
                    destination
                        .locate(&source, column as f64, row as f64)
                        .is_some()
                );
            }
        }
    }

    #[test]
    fn locate_past_right_edge_is_none() {
        let source = north_up();
        let destination = Grid {
            transform: Affine::from_gdal([200.0, 10.0, 0.0, 500.0, 0.0, -10.0]),
            ..source
        };
        assert_eq!(destination.locate(&source, 0.0, 0.0), None);
    }
}
