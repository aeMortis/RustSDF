use rand::Rng;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::data::Data;

pub mod data;

fn generate_rnd_data() -> Data {
    let mut rng = rand::rng();
    Data {
        x: rng.random_range(0.0..=100.0),
        y: rng.random_range(0.0..=100.0),
        z: rng.random_range(0.0..=100.0),
        timestamp: SystemTime::now(),
    }
}

fn create_data_source(consumer_registry: Arc<Mutex<HashMap<usize, mpsc::Sender<Data>>>>) {
    thread::spawn(move || {
        let output_data = generate_rnd_data();

        let registry = consumer_registry.lock().unwrap();
        for client in registry.values() {
            let _ = client.send(output_data.clone());
        }

        thread::sleep(Duration::from_secs(2));
    });
}

fn create_data_consumer(consumer_registry: Arc<Mutex<HashMap<usize, mpsc::Sender<Data>>>>) {
    let mut registry = consumer_registry.lock().unwrap();
    let id = registry.len();
    let (tx, rx) = mpsc::channel();

    registry.insert(id, tx);

    thread::spawn(move || {
        for data in rx {
            println!(
                "Consumer{}: received: {}, {}, {}",
                id,
                data.x,
                data.y,
                data.z // data.timestamp.duration_since(SystemTime::UNIX_EPOCH)
            );
        }
        thread::sleep(Duration::from_secs(1));
    });
}

fn main() {
    println!("Hello RustSDF!");

    let validator_registry: Arc<Mutex<HashMap<usize, mpsc::Sender<Data>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    create_data_consumer(Arc::clone(validator_registry));
    create_data_consumer(Arc::clone(validator_registry));
    create_data_source(Arc::clone(validator_registry));
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
