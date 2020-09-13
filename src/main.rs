mod helpers;
use helpers::{c1, c2, c3, PrintMatrix};

fn main() {
    let val = c1(90.) * c2(-90.);
    val.print();
}

// TODO: Parse Command Line Arguments
// TODO: Route Errors to stderr instead of stdout
// TODO: Write logic in separate file ?
// TODO: Write tests
// TODO: Double check error handling