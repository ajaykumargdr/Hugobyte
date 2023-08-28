pub use solarfam_derive::SolarFam;

#[derive(SolarFam)]
pub struct Earth;

#[derive(SolarFam)]
pub struct Mercury;

#[derive(SolarFam)]
pub struct Venus;

#[derive(SolarFam)]
pub struct Mars;

#[derive(SolarFam)]
pub struct Jupiter;

#[derive(SolarFam)]
pub struct Saturn;

#[derive(SolarFam)]
pub struct Uranus;

#[derive(SolarFam)]
pub struct Neptune;

fn main() {
    let age_in_seconds = 1_000_000_000.0;

    println!("Mercury: {:.2}", Mercury::get_age(age_in_seconds));
    println!("Venus: {:.2}", Venus::get_age(age_in_seconds));
    println!("Earth: {:.2}", Earth::get_age(age_in_seconds));
    println!("Mars: {:.2}", Mars::get_age(age_in_seconds));
    println!("Jupiter: {:.2}", Jupiter::get_age(age_in_seconds));
    println!("Saturn: {:.2}", Saturn::get_age(age_in_seconds));
    println!("Uranus: {:.2}", Uranus::get_age(age_in_seconds));
    println!("Neptune: {:.2}", Neptune::get_age(age_in_seconds));
}