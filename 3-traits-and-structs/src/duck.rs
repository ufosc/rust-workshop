use std::fmt;

pub trait Duck: fmt::Display {
    fn quack(&self) -> String;
    fn start_swiming(&mut self) -> Result<(), &'static str>;
    fn is_swimming(&self) -> bool;
    fn stop_swiming(&mut self) -> Result<(), &'static str>;
}

#[derive(Debug)]
pub struct FlightlessDuck {
    pub name: String,
    pub swimming: bool,
}

#[derive(Debug)]
pub struct FlightfulDuck {
    pub name: String,
    pub swimming: bool,
}

impl fmt::Display for FlightlessDuck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FlightlessDuck named {} and swim status is {}", self.name, self.swimming)
    }
}

impl Duck for FlightlessDuck {

    fn quack(&self) -> String {
        format!("{} says quack but they can't fly :(", self.name)
    }

    fn start_swiming(&mut self) -> Result<(), &'static str> {
        match self.is_swimming() {
            true => Err("Can't start swimming. Already swimming."),
            false => {
                self.swimming = true;
                Ok(())
            }
        }
    }

    fn is_swimming(&self) -> bool {
        self.swimming
    }

    fn stop_swiming(&mut self) -> Result<(), &'static str> {
        match self.is_swimming() {
            false => Err("Can't stop swimming. Not swimming."),
            true => {
                self.swimming = false;
                Ok(())
            }
        }
    }
}


impl Duck for FlightfulDuck {

    fn quack(&self) -> String {
        format!("{} says quack but they also can fly ;)", self.name)
    }

    fn start_swiming(&mut self) -> Result<(), &'static str> {
        match self.is_swimming() {
            true => Err("Can't start swimming. Already swimming."),
            false => {
                self.swimming = true;
                Ok(())
            }
        }
    }

    fn is_swimming(&self) -> bool {
        self.swimming
    }

    fn stop_swiming(&mut self) -> Result<(), &'static str> {
        match self.is_swimming() {
            false => Err("Can't stop swimming. Not swimming."),
            true => {
                self.swimming = false;
                Ok(())
            }
        }
    }
}

impl fmt::Display for FlightfulDuck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FlightfulDuck named {} and swim status is {}", self.name, self.swimming)
    }
}

// impl fmt::Display for dyn Duck {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "A duck with a swim status of {}", self.is_swimming())
//     }
// }
