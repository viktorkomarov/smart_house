use smart_house::dev::{socket::Socket, thermometer::Thermometer};
use smart_house::house::House;
use smart_house::provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

fn main() {
    // Инициализация устройств
    let socket1 = Socket::new("socket1");
    let socket2 = Socket::new("socket2");
    let thermo = Thermometer::new("thermo1");

    // Инициализация дома
    let house = House::new("house");

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let infoprovider1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&infoprovider1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let infoprovider2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&infoprovider2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
