use type_level_example::Car;

fn main() {
    let mut car = Car::new();
    car.into_drive();
    if let Car::Drive(ref mut inner_car) = car {
        inner_car.accelerate(3);
    }
    println!("Speed: {}", car.speed());
    car.honk();
    car.lock_doors();
    car.into_park();
    println!("Speed: {}", car.speed());
}
