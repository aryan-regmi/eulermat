use nalgebra::base::Matrix3;
use super::{c1, c2, c3, PrintMatrix, eig, PrintEig};


pub fn cli() {
    let mut args = std::env::args();
    let num = args.len() - 1;

    // Panic if no arguments
    // Panic if missing rotation-angle pair (ex: c1 90)
    if (num % 2 != 0) | (num <= 0) {
        eprintln!("\nINVALID USE: run with rotational direction, followed by angle in degrees (i.e run with 'c1 90 c2 -90' for C1(90) * C2(-90) ) \n");
        std::process::exit(1)
    }

    // Vector to store all rotation matrices
    let mut results: Vec<Matrix3<f64>> = Vec::new();

    args.next();    // Skip 1st argument (path of file run)

    let mut args = args.peekable();

    // Loop through all arguments
    while args.peek() != None {
        let arg = args.next();
        let angle: f64 = args.next().unwrap().parse().expect("ParseError: Rotation angle could not be parsed!");

        // Match rotation directions (c1, c2, c3)
        match arg {
            Some(rotation) => {
                match &rotation[0..] {
                    "c1" => {
                        results.push(c1(angle));
                    },

                    "c2" =>  {
                        results.push(c2(angle));
                    },

                    "c3" =>  {
                        results.push(c3(angle));
                    },
                    
                    _ => {
                        eprintln!("\nINVALID ROTATION: Please enter a valid rotation direction (c1, c2, c3)\n");
                        std::process::exit(1)
                    }
                }
                
            },
            None => { 
                eprintln!("\nSOMETHING TERRIBLE HAPPENED -- NEVER HIT THIS ERROR!\n");
                std::process::exit(1)
            }
        }
    }

    let res_len = results.len();
    if res_len == 0 {
        eprintln!("\nNo rotations were recorded!\n");
        std::process::exit(1);
    }

    // Multiply each rotational matrix with the previous one (right to left matrix multiplication)
    for i in 0..res_len - 1 {
        results[i+1] = &results[i] * &results[i+1];
    }

    // Print resultant matrix
    results[res_len-1].print();

    // Print Eigen Decomposition
    eig(results[res_len-1]).print();
    
}