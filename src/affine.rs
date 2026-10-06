/// Represents an affine transformation for a raster dataset.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine {
    pub x: f64,            // top left corner of the top left pixel
    pub x_per_column: f64, // pixel width when north-up
    pub x_per_row: f64,    // zero when north-up
    pub y: f64,            // top left corner of the top left pixel
    pub y_per_column: f64, // zero when north-up
    pub y_per_row: f64,    // pixel height when north-up, negative
}

impl Affine {
    /// Maps a pixel position to a world coordinate.
    pub fn xy(&self, column: f64, row: f64) -> (f64, f64) {
        let x = column * self.x_per_column + row * self.x_per_row + self.x;
        let y = column * self.y_per_column + row * self.y_per_row + self.y;
        (x, y)
    }
    /// Maps a world coordinate back to a pixel position, if possible.
    pub fn column_row(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let determinant = self.x_per_column * self.y_per_row - self.x_per_row * self.y_per_column;
        // Zero means the grid has no area, so there is no pixel to hand back.
        if determinant == 0.0 {
            return None;
        }
        let distance_x = x - self.x;
        let distance_y = y - self.y;
        let column = (distance_x * self.y_per_row - distance_y * self.x_per_row) / determinant;
        let row = (distance_y * self.x_per_column - distance_x * self.y_per_column) / determinant;
        Some((column, row))
    }
    /// Maps a pixel of this grid to the matching pixel of source, if possible.
    pub fn locate_in(&self, source: &Affine, column: f64, row: f64) -> Option<(f64, f64)> {
        let (x, y) = self.xy(column, row);
        source.column_row(x, y)
    }
    /// Creates an affine transformation from the six parameters used by rasterio.
    ///
    /// Source: <https://github.com/rasterio/affine/blob/main/src/affine/__init__.py>
    pub fn from_rasterio(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Affine {
            x_per_column: a,
            x_per_row: b,
            x: c,
            y_per_column: d,
            y_per_row: e,
            y: f,
        }
    }
    /// Creates an affine transformation from the six parameters used by GDAL.
    ///
    /// Source: <https://gdal.org/en/stable/user/raster_data_model.html#affine-geotransform>
    pub fn from_gdal(gt: [f64; 6]) -> Self {
        Affine {
            x: gt[0],
            x_per_column: gt[1],
            x_per_row: gt[2],
            y: gt[3],
            y_per_column: gt[4],
            y_per_row: gt[5],
        }
    }
    /// Creates an affine transformation from the GeoTIFF tiepoint and pixel scale arrays.
    ///
    /// Source: <https://docs.ogc.org/is/19-008r4/19-008r4.html#_raster_space>
    pub fn from_geotiff(
        tiepoint: &[f64],
        pixel_scale: &[f64],
        pixel_is_point: bool,
    ) -> Option<Self> {
        if tiepoint.len() < 6 || pixel_scale.len() < 3 {
            return None;
        }
        let mut affine = Affine {
            x: tiepoint[3],
            x_per_column: pixel_scale[0],
            x_per_row: 0.0,
            y: tiepoint[4],
            y_per_column: 0.0,
            y_per_row: -pixel_scale[1],
        };
        if pixel_is_point {
            affine.x -= 0.5 * affine.x_per_column;
            affine.y -= 0.5 * affine.y_per_row;
        }
        Some(affine)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn north_up() -> Affine {
        Affine {
            x: 100.0,
            x_per_column: 10.0,
            x_per_row: 0.0,
            y: 500.0,
            y_per_column: 0.0,
            y_per_row: -10.0,
        }
    }

    #[test]
    fn from_rasterio_matches_struct_literal() {
        assert_eq!(
            Affine::from_rasterio(10.0, 0.0, 100.0, 0.0, -10.0, 500.0),
            north_up()
        );
    }

    #[test]
    fn from_gdal_matches_struct_literal() {
        assert_eq!(
            Affine::from_gdal([100.0, 10.0, 0.0, 500.0, 0.0, -10.0]),
            north_up()
        );
    }

    #[test]
    fn from_gdal_and_from_rasterio_agree() {
        // Same grid, two orderings. The shuffle is the whole point.
        assert_eq!(
            Affine::from_gdal([100.0, 10.0, 0.0, 500.0, 0.0, -10.0]),
            Affine::from_rasterio(10.0, 0.0, 100.0, 0.0, -10.0, 500.0)
        );
    }

    #[test]
    fn from_gdal_keeps_rotation_terms_apart() {
        // Distinct values in every slot, so a swapped pair cannot pass.
        let a = Affine::from_gdal([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(a.x, 1.0);
        assert_eq!(a.x_per_column, 2.0);
        assert_eq!(a.x_per_row, 3.0);
        assert_eq!(a.y, 4.0);
        assert_eq!(a.y_per_column, 5.0);
        assert_eq!(a.y_per_row, 6.0);
    }

    #[test]
    fn from_geotiff_matches_struct_literal() {
        let tiepoint = [0.0, 0.0, 0.0, 100.0, 500.0, 0.0];
        let pixel_scale = [10.0, 10.0, 0.0];
        assert_eq!(
            Affine::from_geotiff(&tiepoint, &pixel_scale, false),
            Some(north_up())
        );
    }

    #[test]
    fn from_geotiff_flips_pixel_height_sign() {
        // The tag stores a positive height; rows run south, so ours is negative.
        let a = Affine::from_geotiff(&[0.0; 6], &[10.0, 10.0, 0.0], false).unwrap();
        assert_eq!(a.y_per_row, -10.0);
    }

    #[test]
    fn from_geotiff_pixel_is_point_shifts_origin_to_corner() {
        let tiepoint = [0.0, 0.0, 0.0, 100.0, 500.0, 0.0];
        let pixel_scale = [10.0, 10.0, 0.0];
        let a = Affine::from_geotiff(&tiepoint, &pixel_scale, true).unwrap();
        // The file said (100, 500) is the centre of pixel (0, 0).
        // The corner is half a pixel up and left of that.
        assert_eq!((a.x, a.y), (95.0, 505.0));
        // So the centre comes back out as the file's own number.
        assert_eq!(a.xy(0.5, 0.5), (100.0, 500.0));
    }

    #[test]
    fn from_geotiff_rejects_short_tags() {
        assert_eq!(
            Affine::from_geotiff(&[0.0; 5], &[10.0, 10.0, 0.0], false),
            None
        );
        assert_eq!(Affine::from_geotiff(&[0.0; 6], &[10.0, 10.0], false), None);
    }

    #[test]
    fn origin_is_pixel_zero_zero() {
        assert_eq!(north_up().xy(0.0, 0.0), (100.0, 500.0));
    }

    #[test]
    fn three_columns_right() {
        assert_eq!(north_up().xy(3.0, 0.0), (130.0, 500.0));
    }

    #[test]
    fn two_rows_down_goes_south() {
        assert_eq!(north_up().xy(0.0, 2.0), (100.0, 480.0));
    }

    #[test]
    fn half_pixel_is_the_centre() {
        assert_eq!(north_up().xy(0.5, 0.5), (105.0, 495.0));
    }

    #[test]
    fn column_row_reverses_xy() {
        let a = north_up();
        let (x, y) = a.xy(3.0, 2.0);
        assert_eq!(a.column_row(x, y), Some((3.0, 2.0)));
    }

    #[test]
    fn column_row_of_origin_is_zero_zero() {
        assert_eq!(north_up().column_row(100.0, 500.0), Some((0.0, 0.0)));
    }

    #[test]
    fn zero_pixel_size_cannot_be_inverted() {
        let flat = Affine {
            x_per_column: 0.0,
            ..north_up()
        };
        assert_eq!(flat.column_row(130.0, 500.0), None);
    }

    /// Turned 90 degrees: columns run north, rows run east.
    fn rotated() -> Affine {
        Affine {
            x: 100.0,
            x_per_column: 0.0,
            x_per_row: 10.0,
            y: 500.0,
            y_per_column: 10.0,
            y_per_row: 0.0,
        }
    }

    #[test]
    fn rotated_xy() {
        assert_eq!(rotated().xy(3.0, 2.0), (120.0, 530.0));
    }

    #[test]
    fn rotated_column_row_reverses_xy() {
        let a = rotated();
        let (x, y) = a.xy(3.0, 2.0);
        assert_eq!(a.column_row(x, y), Some((3.0, 2.0)));
    }

    #[test]
    fn rotated_grid_with_zero_pixel_width_is_still_invertible() {
        assert!(rotated().column_row(120.0, 530.0).is_some());
    }

    #[test]
    fn collapsed_grid_cannot_be_inverted() {
        // Rows and columns point the same way, so the grid has no area.
        let collapsed = Affine {
            x_per_row: 10.0,
            y_per_row: 0.0,
            ..north_up()
        };
        assert_eq!(collapsed.column_row(130.0, 500.0), None);
    }

    #[test]
    fn locate_in_same_grid_is_identity() {
        let a = north_up();
        assert_eq!(a.locate_in(&a, 3.0, 2.0), Some((3.0, 2.0)));
    }

    #[test]
    fn locate_in_grid_shifted_one_pixel_right() {
        let source = north_up();
        let destination = Affine { x: 110.0, ..source };
        assert_eq!(destination.locate_in(&source, 0.0, 0.0), Some((1.0, 0.0)));
    }

    #[test]
    fn locate_in_grid_with_half_size_pixels() {
        let source = north_up();
        let destination = Affine {
            x_per_column: 5.0,
            y_per_row: -5.0,
            ..source
        };
        assert_eq!(destination.locate_in(&source, 4.0, 2.0), Some((2.0, 1.0)));
    }
}
