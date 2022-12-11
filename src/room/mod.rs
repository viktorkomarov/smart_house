pub struct Room<'a> {
    name: String,
    devices: Vec<&'a dyn crate::dev::Device>,
}

impl<'a> Room<'a> {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            devices: Vec::new(),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_dev(&mut self, dev: &'a dyn crate::dev::Device) {
        self.devices.push(dev)
    }

    pub fn devices(&self) -> Vec<&str> {
        self.devices.iter().map(|f| f.get_name()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_name() {
        let expected = "name";
        let room = Room::new(expected);
        assert_eq!(expected, room.get_name());
    }

    #[test]
    fn empty_dev() {
        let room = Room::new("name");
        assert_eq!(true, room.devices().is_empty());
    }

    #[test]
    fn add_and_check() {
        use crate::dev::socket::Socket;
        let mut room = Room::new("name");
        let socket = Socket::new("socket");
        room.add_dev(&socket);

        assert_eq!(vec!["socket"], room.devices());
    }
}
