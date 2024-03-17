# Type-level programming example in Rust
This is a simple example showcasing a powerful concept in Rust sometimes referred to as `Type-level programming`. 

In this example, it's applied to a car. Depending on what gear it's in, we should be permitted to do different things, and some things should be available irrespective of what gear we're in. We'd also like to be able to change gear in a convenient manner. We'll stick with two gears, `Park` and `Neutral`. Some simple designs that don't employ type-level programming to handle this could be:
* Create an `enum Gear` with the variants `Park` and `Drive`, a `struct Car` that among other things contains the field `gear: Gear`, and let the different methods act in different ways depending on the variant in `gear`.
* Create an `enum CarGear` with the variants `Park(Car)` and `Drive(Car)` that wrap around some `struct Car`, where `struct Car` doesn't contain a `gear` field, since this is handled in the variants wrapping `struct Car`: This way, every method will need to implement a `match` statement if it interacts with any data of `struct Car`, even ones that should be available for all variants and should act in identical ways. These `match`s will also be the sole guards for limiting functionality of the different gears.
* One `struct CarGear` for every gear, creating completely different types where the methods they should have in common will need to be individually implemented for every type.

There's another approach, that this example code presents:
* One `trait Gear`
* One struct for each available concrete gear (that is, `Park` and `Drive`), which implements `Gear` in an empty block.
* One generic `struct Car<G: Gear>`, with a `gear: PhantomData` field to keep track of what concrete type were using that implements `Gear`.
* Finally, and optionally if you'd like to have it all wrapped up in one type, an enum that wraps around `struct Car<G: Gear>`, whose variants map to the different types implementing `Gear`.

The optional step of wrapping the generic `Car` inside an enum is done inside `src/lib.rs`, with the rest of the code placed inside `src/struct_car.rs`. A simplified overview of the code:

```rust
use std::marker::PhantomData;

trait Gear {}

struct Park;
struct Drive;
impl Gear for Park {}
impl Gear for Drive {}

struct Car<G: Gear> {
    speed: i32,
    locked_doors: bool,
    gear: PhantomData<G>,
}

// These implementations will only be available for `Park`
impl Car<Park> {
    /// ...
}

// These implementations will only be available for `Drive`
impl Car<Drive> {
    /// ...
}

// These implementations will be available for all types implementing `Gear`
impl<G: Gear> Car<G> { 
    /// ...
}

/// Creating a common type
pub enum CommonEnumCar {
    Park(Car<Park>),
    Drive(Car<Drive>),
}

fn main() {
    let mut car = CommonEnumCar::new();
    car.into_drive();
    if let Car::Drive(ref mut inner_car) = car {
        inner_car.accelerate(3);
    }
    println!("Speed: {}", car.speed()); // Should display `3`
    car.honk();
    car.lock_doors();
    car.into_park();
    println!("Speed: {}", car.speed()); // Should display `0`
}
```

Other sources:
* [Will Crichton](https://willcrichton.net/notes/type-level-programming/)
* [ATSAMD HAL](https://docs.rs/atsamd-hal/latest/atsamd_hal/typelevel/index.html)
