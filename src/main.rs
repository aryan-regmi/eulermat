mod helpers;
use helpers::cli;

fn main() {
    cli();    
}

// TODO: Route Errors to stderr instead of stdout
// TODO: Write tests
// TODO: Use eyre crate for error handling
// TODO: Add Egien vector/value support