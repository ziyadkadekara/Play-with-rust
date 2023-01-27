// Declare Car struct to describe vehicle with four named fields

struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage : u32
}

#[derive(PartialEq, Debug)]

enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String,transmission: Transmission,convertible: bool)-> Car {

    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage : 0
    }
}
fn main() {
let mut car = car_factory(String::from("Red"),Transmission::Manual,false);
println!("Car_1 is {} in color. It has {:?} transmission and it is {}ly convertible and has a mileage of {}",car.color,car.transmission,car.convertible,car.mileage);

car = car_factory(String::from("black"),Transmission::SemiAuto,true);
println!("Car_2 is {} in color. It has {:?} transmission and it is {}ly convertible and has a mileage of {}",car.color,car.transmission,car.convertible,car.mileage);

car = car_factory(String::from("white"),Transmission::Automatic,true);
println!("Car_3 is {} in color. It has {:?} transmission and it is {}ly convertible and has a mileage of {}",car.color,car.transmission,car.convertible,car.mileage);

}
