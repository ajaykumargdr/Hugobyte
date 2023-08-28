use calculate_age::calculate_age;

fn main() {
    let age_in_seconds = 1_000_000_000.0;

    let mercury_age = calculate_age!("Mercury")(age_in_seconds);
    let venus_age = calculate_age!("Venus")(age_in_seconds);
    let earth_age = calculate_age!("Earth")(age_in_seconds);
    let mars_age = calculate_age!("Mars")(age_in_seconds);
    let jupiter_age = calculate_age!("Jupiter")(age_in_seconds);
    let saturn_age = calculate_age!("Saturn")(age_in_seconds);
    let uranus_age = calculate_age!("Uranus")(age_in_seconds);
    let neptune_age = calculate_age!("Neptune")(age_in_seconds);

    println!("Mercury: {:.2}", mercury_age);
    println!("Venus: {:.2}", venus_age);
    println!("Earth: {:.2}", earth_age);
    println!("Mars: {:.2}", mars_age);
    println!("Jupiter: {:.2}", jupiter_age);
    println!("Saturn: {:.2}", saturn_age);
    println!("Uranus: {:.2}", uranus_age);
    println!("Neptune: {:.2}", neptune_age);

}