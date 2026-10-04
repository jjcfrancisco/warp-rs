
/// Represents an affine transformation for a raster dataset.
#[derive(Debug)]
pub struct Affine {
    pub x: f64, // top left corner of the top left pixel
    pub x_per_column: f64, // pixel width when north-up
    pub x_per_row: f64, // zero when north-up
    pub y: f64, // top left corner of the top left pixel
    pub y_per_column: f64, // zero when north-up
    pub y_per_row: f64, // pixel height when north-up, negative
}

impl Affine {
    pub fn xy(&self, column: f64, row: f64) -> (f64, f64) {
        let x = column * self.x_per_column + row * self.x_per_row + self.x;
        let y = column * self.y_per_column + row * self.y_per_row + self.y;
        (x, y)
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
}
