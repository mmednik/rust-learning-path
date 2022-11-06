struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool, mileage: u32) -> Car {
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: mileage
    }
}

fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false, 14000);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Green"), Transmission::SemiAuto, true, 19000);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Blue"), Transmission::Automatic, true, 10000);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
}
