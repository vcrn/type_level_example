pub mod struct_car;

use struct_car::Car as InnerCar;
use struct_car::{Drive, Park};

pub enum Car {
    Park(InnerCar<Park>),
    Drive(InnerCar<Drive>),
}

#[allow(clippy::new_without_default)]
impl Car {
    pub fn new() -> Self {
        Self::Park(InnerCar::new())
    }

    pub fn honk(&self) {
        println!("HONK!");
    }

    pub fn into_drive(&mut self) {
        match self {
            Car::Drive(_) => (),
            Car::Park(car) => *self = Car::Drive(car.into_drive()),
        }
    }

    pub fn into_park(&mut self) {
        match self {
            Car::Drive(car) => *self = Car::Park(car.into_park()),
            Car::Park(_) => (),
        }
    }

    pub fn lock_doors(&mut self) {
        match self {
            Car::Drive(car) => car.lock_doors(),
            Car::Park(car) => car.lock_doors(),
        }
    }

    pub fn unlock_doors(&mut self) {
        match self {
            Car::Drive(car) => car.unlock_doors(),
            Car::Park(car) => car.unlock_doors(),
        }
    }

    pub fn speed(&self) -> i32 {
        match self {
            Car::Drive(car) => car.speed(),
            Car::Park(car) => car.speed(),
        }
    }
}
