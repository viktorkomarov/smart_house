use smart_house::dev::socket::Socket;
use smart_house::house::House;
use smart_house::provider::DeviceInfoProvider;
use smart_house::provider::OwningDeviceInfoProvider;
use smart_house::room::Room;

pub fn main() {
    let mut hs = House::new("name");
    hs.add_room(Room::new("room"));
    let socket = Socket::new("name");
    hs.add_dev("room", &socket);

    let provider = OwningDeviceInfoProvider {
        socket: socket.clone(),
    };

    let actual = provider.create_report(&hs);

    println!("{actual}");
}
