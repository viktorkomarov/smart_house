enum _WattageUnit {
    MW,
    GW,
}

struct _Wattage {
    val: u64,
    unit: _WattageUnit,
}

struct _Socket {}

impl _Socket {
    fn _new() -> Self {
        todo!()
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

enum _TemperatureUnit {
    Fahrenheit,
    Celsius,
}

struct _Temperature {
    val: u64,
    unit: _TemperatureUnit,
}

struct _Termometer {}

impl _Termometer {
    fn _new() -> Self {
        todo!()
    }
    fn _temperature(&self) -> _Temperature {
        todo!()
    }
}

fn main() {}
