use nalgebra::base::Matrix3;
use rust_decimal::prelude::*;
use rust_decimal_macros::*;

trait PrintMatrix {
    fn print(&self);
}

impl PrintMatrix for Matrix3<Decimal> {
    fn print(&self) {
        println!("\n {:.3} , {:.3} , {:.3} \n", self.row(0)[0], self.row(0)[1], self.row(0)[2]);
        println!(" {:.3} , {:.3} , {:.3} \n", self.row(1)[0], self.row(1)[1], self.row(1)[2]);
        println!(" {:.3} , {:.3} , {:.3} \n", self.row(2)[0], self.row(2)[1], self.row(2)[2]);
    }
}

fn cos(theta: Decimal) -> Decimal {
    let temp: f64 = theta.to_f64().unwrap().to_radians().cos();

    Decimal::from_f64(temp).unwrap()
}

fn sin(theta: Decimal) -> Decimal {
    let temp: f64 = theta.to_f64().unwrap().to_radians().sin();

    Decimal::from_f64(temp).unwrap()
}

fn c1(degrees: f64) -> Matrix3<Decimal> {

    let degrees: Decimal = Decimal::from_f64(degrees).unwrap();

    Matrix3::new(
        dec!(1), dec!(0), dec!(0),
        dec!(0), cos(degrees), sin(degrees),
        dec!(0), -sin(degrees), cos(degrees)
    )
}

fn c2(degrees: f64) -> Matrix3<Decimal> {

    let degrees: Decimal = Decimal::from_f64(degrees).unwrap();

    Matrix3::new(
        cos(degrees), dec!(0), -sin(degrees),
        dec!(0), dec!(1), dec!(0),
        sin(degrees), dec!(0), cos(degrees)
    )
}

fn c3(degrees: f64) -> Matrix3<Decimal> {

    let degrees: Decimal = Decimal::from_f64(degrees).unwrap();

    Matrix3::new(
        cos(degrees), sin(degrees), dec!(0),
        -sin(degrees), cos(degrees), dec!(0),
        dec!(0), dec!(0), dec!(1)
    )
}


fn main() {
    let val = c1(90.) * c2(-90.);
    val.print();
}

// TODO: Parse Command Line Arguments
// TODO: Route Errors to stderr instead of stdout
// TODO: Write logic in separate file ?
// TODO: Write tests
// TODO: Double check error handling