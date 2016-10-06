struct Robot {
    uid:           String,
    category:      String,
    battery_level: i32,
}

impl Robot {
    fn new(uid: &str, category: &str, battery_level: i32) -> Robot {
        Robot {
            uid:           uid.to_string(),
            category:      category.to_string(),
            battery_level: battery_level,
        }
    }

    fn beep(&self) {
        println!("beep! uid: {}, category: {}, battery: {}", self.uid, self.category, self.battery_level);
    }
}

fn main() {
    let robot = Robot::new("asdlkj23lkj234", "worker", 100);

    robot.beep();
}
