use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Data {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub timestamp: SystemTime, 
}