pub mod socket;
pub mod thermometer;

pub trait Device {
    fn report(&self) -> String;
    fn get_name(&self) -> &str;
}
