use std::fmt::{Display, Formatter};

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum WattageUnit {
    MW,
    GW,
}

impl Display for WattageUnit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            WattageUnit::MW => write!(f, "mw"),
            WattageUnit::GW => write!(f, "gw"),
        }
    }
}

#[derive(Clone, Copy)]
struct Wattage {
    val: u64,
    unit: WattageUnit,
}

#[derive(Clone)]
pub struct Socket {
    name: String,
    on: bool,
    wattage: Wattage,
}

impl Socket {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            on: false,
            wattage: Wattage {
                val: 0,
                unit: WattageUnit::MW,
            },
        }
    }
    fn _plug_in(&mut self) {
        self.on = true;
    }
    fn _plug_out(&mut self) {
        self.on = false;
    }
    fn _wattage(&self) -> Wattage {
        self.wattage
    }
}

impl super::Device for Socket {
    fn report(&self) -> String {
        if self.on {
            format!(
                "socket {} - {} {}",
                self.name, self.wattage.val, self.wattage.unit
            )
        } else {
            format!("socket {} is off", self.name)
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dev::Device;

    #[test]
    fn valid_name() {
        let name = "name";
        let socket = Socket::new(name);
        assert_eq!(name, socket.get_name())
    }

    #[test]
    fn plug_out_report() {
        let expected = "socket name is off";
        let socket = Socket::new("name");
        assert_eq!(expected, socket.report())
    }

    #[test]
    fn plug_in_report() {
        let expected = "socket name - 0 mw";
        let mut socket = Socket::new("name");
        socket._plug_in();
        assert_eq!(expected, socket.report())
    }
}
