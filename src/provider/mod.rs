use crate::dev::socket::Socket;
use crate::dev::thermometer::Thermometer;
use crate::dev::Device;
use crate::house::House;

pub trait DeviceInfoProvider {
    fn create_report(&self, house: &House) -> String;
}

pub struct OwningDeviceInfoProvider {
    pub socket: Socket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn create_report(&self, house: &House) -> String {
        build_report(house, &self.socket)
    }
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a Socket,
    pub thermo: &'b Thermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn create_report(&self, house: &House) -> String {
        let mut report = build_report(house, self.socket);
        report.push_str(build_report(house, self.thermo).as_str());
        report
    }
}

fn build_report(house: &House, dev: &dyn Device) -> String {
    let rooms = house.get_rooms();

    let mut report = String::new();
    rooms.iter().for_each(|f| {
        if let Ok(loc) = house.dev_location(f, dev.get_name()) {
            report.push_str(
                format!(
                    "Device was detected in {}, {}, dev: {}\n",
                    loc.house,
                    loc.room,
                    dev.report()
                )
                .as_str(),
            )
        }
    });
    report
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dev::socket::Socket;
    use crate::house::House;
    use crate::room::Room;

    #[test]
    fn simple_report() {
        let mut hs = House::new("name");
        hs.add_room(Room::new("room"));
        let socket = Socket::new("name");
        assert!(matches!(hs.add_dev("room", &socket), Ok(())));

        let provider = OwningDeviceInfoProvider {
            socket: socket.clone(),
        };

        let actual = provider.create_report(&hs);

        assert_eq!(
            format!("Device was detected in name, room, dev: socket name is off\n").to_string(),
            actual
        )
    }
}
