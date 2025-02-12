use std::path::Path;

use rticonnector::RTIConnector;




struct Square {

}

struct Circle {

}

struct Triangle {

}

fn main() {
    let shape_example_xml = Path::new("examples/ShapeExample.xml").to_str().unwrap();

    let connector = RTIConnector::new("ShapeDemo", shape_example_xml, &[]);
    
}