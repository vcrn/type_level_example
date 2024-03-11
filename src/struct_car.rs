use std::marker::PhantomData;

pub trait Gear {}
#[derive(Copy, Clone)]
pub struct Park;
#[derive(Copy, Clone)]
pub struct Drive;

impl Gear for Park {}
impl Gear for Drive {}

#[derive(Copy, Clone)]
pub struct Car<G: Gear> {
    speed: i32,
    locked_doors: bool,
    gear: PhantomData<G>,
}

// These implementations will only be available for `Park`
impl Car<Park> {
    /// Shifts gear into `Drive`
    pub fn into_drive(self) -> Car<Drive> {
        Car {
            speed: self.speed,
            locked_doors: self.locked_doors,
            gear: PhantomData,
        }
    }
}

// These implementations will only be available for `Drive`
impl Car<Drive> {
    pub fn accelerate(&mut self, increase: u32) {
        self.speed += increase as i32;
    }

    pub fn slow_down(&mut self, decrease: u32) {
        self.speed -= decrease as i32;
    }

    /// Shifts gear into `Park`
    #[must_use]
    pub fn into_park(self) -> Car<Park> {
        Car {
            speed: 0,
            locked_doors: self.locked_doors,
            gear: PhantomData,
        }
    }
}

// These implementations will be available for all types implementing `Gear`
#[allow(clippy::new_without_default)]
impl<G: Gear> Car<G> {
    pub fn new() -> Car<G> {
        Car {
            speed: 0,
            locked_doors: true,
            gear: PhantomData,
        }
    }

    pub fn lock_doors(&mut self) {
        self.locked_doors = true;
    }

    pub fn unlock_doors(&mut self) {
        self.locked_doors = false;
    }

    pub fn speed(&self) -> i32 {
        self.speed
    }

    pub fn are_doors_locked(&self) -> bool {
        self.locked_doors
    }
}
