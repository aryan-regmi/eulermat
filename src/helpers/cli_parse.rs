use nalgebra::base::Matrix3;


pub fn cli() -> Result<(), String> {
    let mut args = std::env::args();
    let num = args.len() - 1;

    // Panic if no arguments
    // Panic if missing rotation-angle pair (ex: c1 90)
    if (num % 2 != 0) | (num <= 0) {
        panic!("Invalid use: run with '--help' for usage information.")
    }

    let results: Vec<Matrix3<f64>> = Vec::new();

    args.next();
    let mut args = args.peekable();

    while args.peek() != None {
        let arg = args.next();

        match arg {
            Some(rotation) => {
                match &rotation[0..] {
                    "c1" => unimplemented!(),
                    "c2" => unimplemented!(),
                    "c3" => unimplemented!(),
                    "--help" => unimplemented!(),
                    _ => unimplemented!()
                }
                
            },
            None => panic!("SOMETHING TERRIBLE HAPPENED -- NEVER HIT THIS ERROR!")
        }
    }

    unimplemented!()
}