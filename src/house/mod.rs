use crate::provider::DeviceInfoProvider;
use crate::room::Room;

#[derive(Debug)]
pub enum SmartHouseError {
    ErrNotFound,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Location {
    pub house: String,
    pub room: String,
}

pub struct House<'a> {
    name: String,
    rooms: Vec<Room<'a>>,
}

impl<'a> House<'a> {
    pub fn new(name: &str) -> Self {
        House {
            name: String::from(name),
            rooms: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|f| f.get_name()).collect()
    }

    pub fn add_room(&mut self, room: Room<'a>) {
        self.rooms.push(room);
    }

    pub fn add_dev(
        &mut self,
        name: &str,
        dev: &'a dyn crate::dev::Device,
    ) -> Result<(), SmartHouseError> {
        for room in &mut self.rooms {
            if room.get_name() == name {
                room.add_dev(dev);
                return Ok(());
            }
        }
        Err(SmartHouseError::ErrNotFound)
    }

    pub fn devices(&self, room: &str) -> Result<Vec<&str>, SmartHouseError> {
        for rm in &self.rooms {
            if room == rm.get_name() {
                return Ok(rm.devices());
            }
        }
        Err(SmartHouseError::ErrNotFound)
    }

    pub fn dev_location(&self, room: &str, dev: &str) -> Result<Location, SmartHouseError> {
        match self.devices(room) {
            Ok(devs) => {
                for dv in devs {
                    if dv == dev {
                        return Ok(Location {
                            house: String::from(self.get_name()),
                            room: String::from(room),
                        });
                    }
                }
                Err(SmartHouseError::ErrNotFound)
            }
            Err(err) => Err(err),
        }
    }

    pub fn create_report(&self, reporter: &dyn DeviceInfoProvider) -> String {
        reporter.create_report(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_name() {
        let expected = "name";
        let room = House::new(expected);
        assert_eq!(expected, room.get_name());
    }

    #[test]
    fn empty_rooms() {
        let hs = House::new("name");
        assert_eq!(true, hs.get_rooms().is_empty());
    }

    #[test]
    fn add_and_check() {
        use crate::room::Room;
        let mut hs = House::new("name");
        hs.add_room(Room::new("room"));

        assert_eq!(vec!["room"], hs.get_rooms());
    }

    #[test]
    fn add_dev() {
        use crate::dev::socket::Socket;
        use crate::room::Room;

        let mut hs = House::new("name");
        hs.add_room(Room::new("room"));
        let socket = Socket::new("socket");
        let err = hs.add_dev("room2", &socket).err().unwrap();
        assert!(matches!(
            hs.add_dev("room2", &socket),
            Err(SmartHouseError::ErrNotFound)
        ));
        assert!(matches!(hs.add_dev("room", &socket), Ok(())));

        assert_eq!(vec!["socket"], hs.devices("room").unwrap());
        assert_eq!(
            Location {
                house: "name".to_string(),
                room: "room".to_string(),
            },
            hs.dev_location("room", "socket").unwrap(),
        );
    }
}
