#[derive(Debug)]
pub enum SignalLights {
    RED(u8),
    GREEN(u8),
    YELLOW(u8),
}

pub trait Signal {
    fn signal_time(&self) -> u8;
}

impl Signal for SignalLights {
    fn signal_time(&self) -> u8 {
        match *self {
            SignalLights::RED(red_time) => red_time,
            SignalLights::GREEN(green_time) => green_time,
            SignalLights::YELLOW(yellow_time) => yellow_time,
        }
    }
}
