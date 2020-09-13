use nalgebra::base::Matrix3;

trait PrintMatrix {
    fn print(&self);
}

impl PrintMatrix for Matrix3<f64> {
    fn print(&self) {
        println!("\n {:#?} , {:#?} , {:#?} \n", self.row(0)[0], self.row(0)[1], self.row(0)[2]);
        println!(" {:#?} , {:#?} , {:#?} \n", self.row(1)[0], self.row(1)[1], self.row(1)[2]);
        println!(" {:#?} , {:#?} , {:#?} \n", self.row(2)[0], self.row(2)[1], self.row(2)[2]);
    }
}




fn main() {
    let m1 = Matrix3::new(1.,0.,0.,
                                                   0.,0.,1.,
                                                   0.,-1.,0.);
    let m2 = Matrix3::new(0.,0.,1.,
                                                  0.,1.,0.,
                                                  -1.,0.,0.);

    let prod = m1 * m2;
    // println!("{:#?}", prod.data[0]);
    prod.print();
}


// TODO: Route Errors to stderr instead of stdout
// TODO: Write logic in separate file ?
// TODO: Write tests
// TODO: Double check error handling