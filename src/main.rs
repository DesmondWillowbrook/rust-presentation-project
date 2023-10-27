use std::{fs::File, io::{Read, BufRead, BufReader}, str::FromStr};

struct Measurement {
	temperature: f64,
	time: u64,
}

impl Measurement {
	fn new(temperature: f64, time: u64) -> Measurement {
		Measurement {
			temperature,
			time,
		}
	}
}

impl FromStr for Measurement {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
        // the expr? operator is shorthand for:
		// match expr() {
		// 	Ok(val) => val,
		// 	Err(err) => return Err(err),
		// }

        // the .ok() method is shorthand for:
        // match expr {
        //     Some(val) => Ok(val),
        //     None => Err(()),
        // }
        // that is, it converts a Result into an Option

		let mut parts = s.split_whitespace();
		let temperature: f64 = parts.next().ok_or("No temperature")?.parse().map_err(|_| "Invalid temperature")?;

		let time: u64 = parts.next().ok_or("No time")?.parse().map_err(|_| "Invalid time")?;

		todo!("return a Measurement using the `new` method")
	}
}

pub struct MeasurementReader<T: BufRead> {
    elapsed_time: u64,
	total_temperature: f64,
    reader: T,
}

impl<T: BufRead> MeasurementReader<T> {
    pub fn new(reader: T) -> MeasurementReader<T> {
        todo!("create and return MeasurementReader using the given reader")
	}
}

impl<T: BufRead> Iterator for MeasurementReader<T> {
    type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		let mut line = String::new();
		todo!("call read_line on the reader, and return None if reader is empty using the ? operator");

		let measurement: Measurement = line.parse().ok()?;

		self.elapsed_time += measurement.time;
		self.total_temperature += measurement.temperature;

		Some(self.total_temperature / self.elapsed_time as f64)
	}
}


fn main () {
	let f = File::open("example.txt");

	let f: File = todo!("Check whether f is a file or an error using the `expect` method");

	let reader = BufReader::new(f);
	MeasurementReader::new(reader).for_each(|x| println!("{}", x));
}