use waybar_server_logic::{WayBarJsonObj};

const ARRAY_SIZE :usize= 100;
//TODO, make it work for more then just one battery. new() parameter?
const FILE_PATH :&str= "/sys/class/power_supply/BAT0/";
///This struct represents A battery. 
pub struct Battery{
    capacity : u8,
    charge_now : u32,
    charge_full : u32,
    current_now : u32,
    current_avg_array : arraydeque::ArrayDeque<[u32;ARRAY_SIZE],arraydeque::behavior::Wrapping>,
    current_avg : u32,
}

impl Battery{
    ///Generates new Battery struct, 
    pub fn new() -> Battery {
        let charge_now = Battery::read_error_corrected(&(FILE_PATH.to_string() + "charge_now"));
        let charge_now :u32= charge_now.trim().parse().unwrap();
        let charge_full = Battery::read_error_corrected(&(FILE_PATH.to_string() + "charge_full"));
        let charge_full :u32= charge_full.trim().parse().unwrap();
        let current_now = Battery::read_error_corrected(&(FILE_PATH.to_string() + "current_now"));
        let current_now :u32 = current_now.trim().parse().unwrap();
        let capacity :u8= (charge_now as f64/charge_full as f64 * 100 as f64).round() as u8;
        let mut current_avg_array : arraydeque::ArrayDeque<[u32;ARRAY_SIZE],arraydeque::behavior::Wrapping> = arraydeque::ArrayDeque::new();
        for _i in 0..ARRAY_SIZE as u32 {
            current_avg_array.push_front(current_now);
        }
        println!("{:?}",current_avg_array);
        Battery {
            capacity : capacity,
            charge_now : charge_now,
            charge_full : charge_full,
            current_now : current_now,
            current_avg_array : current_avg_array,
            current_avg : current_now,
        }
    }
    ///Shouldn't *need* to be called publicly, but can inorder to manually update to most latest data.
    pub fn update (&mut self) -> &Battery{
        let charge_now = Battery::read_error_corrected(&(FILE_PATH.to_string() + "charge_now"));
        let charge_now :u32= charge_now.trim().parse().unwrap();
        let charge_full = Battery::read_error_corrected(&(FILE_PATH.to_string() + "charge_full"));
        let charge_full :u32= charge_full.trim().parse().unwrap();
        let current_now = Battery::read_error_corrected(&(FILE_PATH.to_string()+ "current_now"));
        let current_now :u32 = current_now.trim().parse().unwrap();
        let capacity :u8= (charge_now as f64/charge_full as f64 * 100 as f64).round() as u8;
        self.current_avg_array.push_front(current_now);
        self.charge_now = charge_now;
        self.charge_full = charge_full;
        self.current_now = current_now;
        self.capacity = capacity;
        self.current_avg = self.current_avg_array.iter().sum::<u32>() / ARRAY_SIZE as u32;
        self
    }
    ///Gets current current 
    pub fn current_now(&mut self) -> u32 {
        Battery::update(self);
        self.current_now
    }
    ///Gets current capacity as percentage
    pub fn capacity(&mut self) -> u8 {
        Battery::update(self);
        self.capacity
    }
    ///Gets current charge level
    pub fn charge_now(&mut self) -> u32 {
        Battery::update(self);
        self.charge_now
    }
    ///Gets max charge level 
    pub fn charge_full (&mut self)->u32{
        Battery::update(self);
        self.charge_full
    }
    ///Gets average current
    pub fn current_avg(&mut self) ->u32 {
        Battery::update(self);
        self.current_avg
    }
    ///Generates a WayBarJsonObj with battery info, namely percentage as text and percentage field,
    /// along with time left in tooltip.
    pub fn battery_indicator(&mut self) -> WayBarJsonObj {
        Battery::update(self);
        let mut waybarblock = waybar_server_logic::WayBarJsonObj::default();
        waybarblock.text = self.capacity.to_string();
        waybarblock.percentage = Some(self.capacity);
        let battery_life_hours = self.charge_now as f64 / (self.current_avg as f64*100f64 * 0.01f64 ) *0.7f64;
        println!("{}",battery_life_hours);
        waybarblock.tooltip = Some(format!(
            "{capacity}% Remaining, or about {battery_life_hours:.0} hours and {battery_life_minutes:.0} minutes."
            ,capacity = self.capacity,
            battery_life_hours = battery_life_hours as u32,
            battery_life_minutes = (battery_life_hours - battery_life_hours as u32 as f64) * 60f64, 
        ));
        waybarblock
    }

    ///uses std::fs::read_to_string, but retries after 1 second on errors.
    fn read_error_corrected(path: &str) -> String{
        loop {
            match std::fs::read_to_string(path) {
                Ok(t) => return t,
                Err(_) => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                },
            }
        }
    }
}


