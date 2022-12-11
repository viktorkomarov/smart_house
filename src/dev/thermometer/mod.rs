use std::fmt::{Display, Formatter};

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum TemperatureUnit {
    Fahrenheit,
    Celsius,
}

impl Display for TemperatureUnit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TemperatureUnit::Fahrenheit => write!(f, "f"),
            TemperatureUnit::Celsius => write!(f, "c"),
        }
    }
}

#[derive(Clone, Copy)]
struct Temperature {
    val: u64,
    unit: TemperatureUnit,
}

pub struct Thermometer {
    name: String,
    temp: Temperature,
}

impl Thermometer {
    pub fn new(name: &str) -> Self {
        Thermometer {
            name: String::from(name),
            temp: Temperature {
                val: 0,
                unit: TemperatureUnit::Celsius,
            },
        }
    }
    fn _temperature(&self) -> Temperature {
        self.temp
    }
}

impl super::Device for Thermometer {
    fn report(&self) -> String {
        format!(
            "thermometer {} - {} {}",
            self.name, self.temp.val, self.temp.unit,
        )
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use crate::dev::Device;

    use super::*;

    #[test]
    fn valid_name() {
        let name = "name";
        let dev = Thermometer::new(name);
        assert_eq!(name, dev.get_name())
    }

    #[test]
    fn report() {
        let expected = "thermometer name - 0 c";
        let socket = Thermometer::new("name");
        assert_eq!(expected, socket.report())
    }
}
