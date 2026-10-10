//! Prints one JSON line per case so `scripts/cross_check.py` can compare
//! our numbers with the `affine` package and with GDAL.

use warp_rs::affine::Affine;
use warp_rs::grid::Grid;

fn geotransform(affine: &Affine) -> String {
    format!(
        "[{:?}, {:?}, {:?}, {:?}, {:?}, {:?}]",
        affine.x,
        affine.x_per_column,
        affine.x_per_row,
        affine.y,
        affine.y_per_column,
        affine.y_per_row
    )
}

fn pair(value: Option<(f64, f64)>) -> String {
    match value {
        Some((a, b)) => format!("[{:?}, {:?}]", a, b),
        None => "null".to_string(),
    }
}

fn main() {
    let grids: Vec<(&str, Grid)> = vec![
        (
            "north_up",
            Grid {
                width: 10,
                height: 8,
                transform: Affine::from_gdal([100.0, 10.0, 0.0, 500.0, 0.0, -10.0]),
            },
        ),
        (
            "south_up",
            Grid {
                width: 10,
                height: 8,
                transform: Affine::from_gdal([100.0, 10.0, 0.0, 420.0, 0.0, 10.0]),
            },
        ),
        (
            "quarter_turn",
            Grid {
                width: 10,
                height: 8,
                transform: Affine::from_gdal([100.0, 0.0, 10.0, 500.0, 10.0, 0.0]),
            },
        ),
        (
            "rotated_30_degrees",
            Grid {
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
            },
        ),
        (
            "half_size_pixels",
            Grid {
                width: 20,
                height: 16,
                transform: Affine::from_gdal([100.0, 5.0, 0.0, 500.0, 0.0, -5.0]),
            },
        ),
        (
            "shifted_right",
            Grid {
                width: 10,
                height: 8,
                transform: Affine::from_gdal([200.0, 10.0, 0.0, 500.0, 0.0, -10.0]),
            },
        ),
        (
            "geographic",
            Grid {
                width: 3600,
                height: 1800,
                transform: Affine::from_gdal([-180.0, 0.1, 0.0, 90.0, 0.0, -0.1]),
            },
        ),
    ];

    let points: [(f64, f64); 6] = [
        (0.0, 0.0),
        (0.5, 0.5),
        (3.0, 2.0),
        (9.5, 7.5),
        (10.0, 8.0),
        (-1.25, 3.75),
    ];

    for (name, grid) in &grids {
        let (left, bottom, right, top) = grid.bounds();
        println!(
            "{{\"kind\": \"bounds\", \"grid\": \"{}\", \"width\": {}, \"height\": {}, \"geotransform\": {}, \"result\": [{:?}, {:?}, {:?}, {:?}]}}",
            name,
            grid.width,
            grid.height,
            geotransform(&grid.transform),
            left,
            bottom,
            right,
            top
        );

        for (column, row) in points {
            let (x, y) = grid.transform.xy(column, row);
            println!(
                "{{\"kind\": \"xy\", \"grid\": \"{}\", \"geotransform\": {}, \"column\": {:?}, \"row\": {:?}, \"result\": [{:?}, {:?}]}}",
                name,
                geotransform(&grid.transform),
                column,
                row,
                x,
                y
            );
            println!(
                "{{\"kind\": \"column_row\", \"grid\": \"{}\", \"geotransform\": {}, \"x\": {:?}, \"y\": {:?}, \"result\": {}}}",
                name,
                geotransform(&grid.transform),
                x,
                y,
                pair(grid.transform.column_row(x, y))
            );
            println!(
                "{{\"kind\": \"contains\", \"grid\": \"{}\", \"width\": {}, \"height\": {}, \"column\": {:?}, \"row\": {:?}, \"result\": {}}}",
                name,
                grid.width,
                grid.height,
                column,
                row,
                grid.contains(column, row)
            );
        }
    }

    for (destination_name, destination) in &grids {
        for (source_name, source) in &grids {
            for (column, row) in points {
                println!(
                    "{{\"kind\": \"locate\", \"destination\": \"{}\", \"source\": \"{}\", \"destination_geotransform\": {}, \"source_geotransform\": {}, \"source_width\": {}, \"source_height\": {}, \"column\": {:?}, \"row\": {:?}, \"result\": {}}}",
                    destination_name,
                    source_name,
                    geotransform(&destination.transform),
                    geotransform(&source.transform),
                    source.width,
                    source.height,
                    column,
                    row,
                    pair(destination.locate(source, column, row))
                );
            }
        }
    }
}
