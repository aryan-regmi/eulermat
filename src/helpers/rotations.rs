use nalgebra::base::{Matrix3};


pub trait PrintMatrix {
    fn print(&self);
}

impl PrintMatrix for Matrix3<f64> {
    fn print(&self) {
        println!("\nRotation Matrix:");
        println!("------------------");
        println!(" {:.3} , {:.3} , {:.3} \n", self.row(0)[0], self.row(0)[1], self.row(0)[2]);
        println!(" {:.3} , {:.3} , {:.3} \n", self.row(1)[0], self.row(1)[1], self.row(1)[2]);
        println!(" {:.3} , {:.3} , {:.3} \n", self.row(2)[0], self.row(2)[1], self.row(2)[2]);
    }
}

pub fn c1(degrees: f64) -> Matrix3<f64> {

    let degrees = degrees.to_radians();

    Matrix3::new(
        1., 0., 0.,
        0., degrees.cos(), degrees.sin(),
        0., -degrees.sin(), degrees.cos()
    )
}

pub fn c2(degrees: f64) -> Matrix3<f64> {

    let degrees = degrees.to_radians();

    Matrix3::new(
        degrees.cos(), 0., -degrees.sin(),
        0., 1., 0.,
        degrees.sin(), 0., degrees.cos()
    )
}

pub fn c3(degrees: f64) -> Matrix3<f64> {

    let degrees = degrees.to_radians();

    Matrix3::new(
        degrees.cos(), degrees.sin(), 0.,
        -degrees.sin(), degrees.cos(), 0.,
        0., 0., 1.
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use speculate::speculate;
    speculate! {
        describe "Rotation Matrix" {

            before {
                let _c1_90 = Matrix3::new(
                    1., 0., 0.,
                    0., 0.00000000000000006123233995736766, 1.,
                    0., -1., 0.00000000000000006123233995736766
                );

                let _c2_90 = Matrix3::new(
                    0.00000000000000006123233995736766, 0., 1.,
                    0., 1., 0.,
                    -1., 0., 0.00000000000000006123233995736766
                );

                let _c1c2 = Matrix3::new(
                    0.00000000000000006123233995736766, 0., 1.0,
                    -1., 0.00000000000000006123233995736766, 0.00000000000000006123233995736766,
                    -0.00000000000000006123233995736766, -1., 0.000000000000000000000000000000003749399456654644
                );
            }

            it "can do C1 rotation" {
                assert_eq!(c1(90.), _c1_90)
            }

            it "can do C2 rotation" {
                assert_eq!(c2(-90.), _c2_90)           
            }
            
            it "can multiply rotations" {
                println!("HERE: {:#?}", c1(90.) * c2(-90.));
                assert_eq!(c1(90.) * c2(-90.), _c1c2)
            }
        }
    }
}