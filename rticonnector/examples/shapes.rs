use std::path::Path;

use rticonnector::Connector;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct ShapeType {
    color: String,
    x: i32,
    y: i32,
    shapesize: i32,
}
#[derive(Serialize, Deserialize)]
enum ShapeFillKind {
    SOLID_FILL = 0,
    TRANSPARENT_FILL = 1,
    HORIZONTAL_HATCH_FILL = 2,
    VERTICAL_HATCH_FILL = 3,
}

impl Default for ShapeFillKind {
    fn default() -> Self {
        ShapeFillKind::SOLID_FILL
    }
}
#[derive(Serialize, Deserialize, Default)]
struct ShapeTypeExtended {
    baseType: ShapeType,
    fillKind: ShapeFillKind,
    angle: f32,
}

fn main() {
    let shape_example_xml = Path::new("examples/ShapeExample.xml").to_str().unwrap();

    let mut connector = Connector::new("ShapeDemo", shape_example_xml, &[]);

    // note that the data reader needs the same name from the `ShapeExample.xml` file.
    let datareader = connector.get_dynamic_datareader("MySquareReader").unwrap();
    let datawriter = connector.get_dynamic_datawriter("MySquareWriter").unwrap();

    connector
        .write(
            "Square",
            &serde_json::to_string::<ShapeTypeExtended>(&ShapeTypeExtended::default()).unwrap(),
        )
        .unwrap();
}
