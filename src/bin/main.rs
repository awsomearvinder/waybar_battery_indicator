mod battery;
fn main() {
    let mut battery = battery::Battery::new();
    loop {
        battery.battery_indicator().send().expect("Don't expect this to error rn.");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

