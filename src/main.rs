use std::fmt::{Display, Formatter};

enum SmartHouseError {
    ErrNotFound,
}

struct House<'a> {
    name: String,
    rooms: Vec<Room<'a>>,
}

impl<'a> House<'a> {
    fn _new(name: &str) -> Self {
        House {
            name: String::from(name),
            rooms: Vec::new(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|f| f.get_name()).collect()
    }

    fn devices(&self, room: &str) -> Result<Vec<&str>, SmartHouseError> {
        for rm in &self.rooms {
            if room == rm.get_name() {
                return Ok(rm.devices());
            }
        }
        Err(SmartHouseError::ErrNotFound)
    }

    fn dev_location(&self, room: &str, dev: &str) -> Result<Location, SmartHouseError> {
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

    fn _create_report(&self, reporter: &dyn DeviceInfoProvider) -> String {
        reporter.create_report(self)
    }
}

struct Room<'a> {
    name: String,
    devices: Vec<&'a dyn Device>,
}

impl<'a> Room<'a> {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn devices(&self) -> Vec<&str> {
        self.devices.iter().map(|f| f.get_name()).collect()
    }
}

trait Device {
    fn report(&self) -> String;
    fn get_name(&self) -> &str;
}

enum _WattageUnit {
    _MW,
    _GW,
}

impl Display for _WattageUnit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            _WattageUnit::_MW => write!(f, "mw"),
            _WattageUnit::_GW => write!(f, "gw"),
        }
    }
}

struct _Wattage {
    val: u64,
    unit: _WattageUnit,
}

struct _Socket {
    name: String,
    on: bool,
    wattage: _Wattage,
}

impl _Socket {
    fn _new(name: &str) -> Self {
        Self {
            name: String::from(name),
            on: false,
            wattage: _Wattage {
                val: 0,
                unit: _WattageUnit::_MW,
            },
        }
    }
    fn _plug_in(&mut self) {
        todo!()
    }
    fn _plug_out(&mut self) {
        todo!()
    }
    fn _wattage(&self) -> _Wattage {
        todo!()
    }
    fn _desc(&self) -> String {
        todo!()
    }
}

impl Device for _Socket {
    fn report(&self) -> String {
        format!(
            "socket {}, on {}, {} {}",
            self.name, self.on, self.wattage.val, self.wattage.unit
        )
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

enum _TemperatureUnit {
    _Fahrenheit,
    _Celsius,
}

impl Display for _TemperatureUnit {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            _TemperatureUnit::_Fahrenheit => write!(f, "f"),
            _TemperatureUnit::_Celsius => write!(f, "c"),
        }
    }
}

struct _Temperature {
    val: u64,
    unit: _TemperatureUnit,
}

struct _Termometer {
    name: String,
    temp: _Temperature,
}

impl _Termometer {
    fn _new(name: &str) -> Self {
        _Termometer {
            name: String::from(name),
            temp: _Temperature {
                val: 0,
                unit: _TemperatureUnit::_Celsius,
            },
        }
    }
    fn _temperature(&self) -> _Temperature {
        todo!()
    }
}

impl Device for _Termometer {
    fn report(&self) -> String {
        format!(
            "termometer {}, {} {}",
            self.name, self.temp.val, self.temp.unit,
        )
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
struct Location {
    house: String,
    room: String,
}

trait DeviceInfoProvider {
    fn create_report(&self, house: &House) -> String;
}

struct OwningDeviceInfoProvider {
    socket: _Socket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn create_report(&self, house: &House) -> String {
        build_report(house, &self.socket)
    }
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a _Socket,
    thermo: &'b _Termometer,
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
                    "Device was detected in {}, {}, state: {}\n",
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

fn main() {
    // Инициализация устройств
    let socket1 = _Socket::_new("socket_1");
    let socket2 = _Socket::_new("socket_2");
    let thermo = _Termometer::_new("thermo_1");

    // Инициализация дома
    let house = House::_new("house");

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house._create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house._create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
