use nalgebra::base::{Matrix3, Matrix};
use nalgebra::ArrayStorage;
use nalgebra::base::dimension::U3 as U3;
use nalgebra::base::dimension::U1 as U1;
use nalgebra::Complex;

type CompVal = Complex<f64>;

pub trait PrintEig {
    fn print(&self);
}

impl PrintEig for Matrix<CompVal, U3, U1, ArrayStorage<CompVal, U3, U1>> {
    fn print(&self) {
        let data_len = self.data.len();
        println!("\nEigen Values:");
        println!("---------------");
        for i in 0..data_len {
            if self.data[i].im < 0. {
                println!("{:.3} - {:.3}i", self.data[i].re, self.data[i].im.abs());
            }
            else {
                println!("{:.3} + {:.3}i", self.data[i].re, self.data[i].im);
            }           
        }; 
        println!("\n");      
    }
}

pub fn eig(rot_mat: Matrix3<f64>) -> Matrix<CompVal, U3, U1, ArrayStorage<CompVal, U3, U1>> {
    let eigen_values = rot_mat.complex_eigenvalues();
    
    eigen_values
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use speculate::speculate;
    speculate! {
        describe "Eigen Values" {

            ///Function to round Vec<f64> to 3 decimal places
            fn round3(list: Vec<f64>) -> Vec<f64> {
                let mut out = Vec::new();
                for num in list.iter() {
                    out.push((num * 1000.0).round() / 1000.0);
                };
                out                
            }

            before {
                let _rotmat = Matrix3::new(
                    0., 0., -1.,
                    1., 0., 0.,
                    0., -1., 0.
                );
                let _eig_vals = eig(_rotmat);
                let _reals = round3(vec![_eig_vals.data[0].re, _eig_vals.data[1].re, _eig_vals.data[2].re]);
                let _imaginary = round3(vec![_eig_vals.data[0].im, _eig_vals.data[1].im, _eig_vals.data[2].im]);
            }

            it "calculates eigenvalue reals" {
                assert_eq!(_reals, [-0.5, -0.5, 1.])
            }
            it "calculates eigenvalue imaginaries" {
                assert_eq!(_imaginary, [0.866, -0.866, 0.])
            }

        }
    }
}