use std::path::Path;

use rticonnector::Connector;




struct Square {

}

struct Circle {

}

struct Triangle {

}

fn main() {
    let shape_example_xml = Path::new("examples/ShapeExample.xml").to_str().unwrap();

    let connector = Connector::new("ShapeDemo", shape_example_xml, &[]);
    
}