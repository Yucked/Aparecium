use umbriel::PartialUpdater;
use umbriel_tests::{Rocket, Weapon};

// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;

fn main() {
    let rocket = Rocket::partial_builder()
        .strength(907)
        .name("Ye".into())
        .build();
    let x = Weapon::partial_builder()
        .name("Charlie".into())
        .strength(45.0)
        .rocket(rocket)
        .build();
    // Weapon::pa
    println!("{:?}", serde_json::to_string(&x).unwrap());
}
