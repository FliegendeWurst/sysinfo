use std::{env, fs, time::Duration};

const WAIT_MS: u64 = 500;

fn main() {
	let temp_input = env::args()
		.nth(1)
		.unwrap_or_else(|| "/sys/class/hwmon/hwmon2/temp1_input".to_owned());

	let stat_1 = extract_stat();
	std::thread::sleep(Duration::from_millis(WAIT_MS));
	let stat_2 = extract_stat();
	let real_load = (stat_2 - stat_1) * 1000 / (WAIT_MS as u64);
	let temp = fs::read_to_string(temp_input).unwrap().trim().parse::<u64>().unwrap() / 1000;
	let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
	let meminfo = meminfo.split('\n').collect::<Vec<_>>();
	let total = meminfo
		.iter()
		.filter(|x| x.starts_with("MemTotal"))
		.next()
		.unwrap()
		.split_whitespace()
		.nth(1)
		.unwrap()
		.parse::<u64>()
		.unwrap()
		/ 1_000;
	let available = meminfo
		.iter()
		.filter(|x| x.starts_with("MemAvailable"))
		.next()
		.unwrap()
		.split_whitespace()
		.nth(1)
		.unwrap()
		.parse::<u64>()
		.unwrap()
		/ 1_000;
	let dirty = meminfo
		.iter()
		.filter(|x| x.starts_with("Dirty"))
		.next()
		.unwrap()
		.split_whitespace()
		.nth(1)
		.unwrap()
		.parse::<u64>()
		.unwrap()
		/ 1_000;
	if dirty > 100 {
		println!(
			"{}% {:.1}°C {}/{}MB dirty={}MB",
			real_load,
			temp,
			total - available,
			total,
			dirty
		);
	} else {
		println!("{}% {:.1}°C {}/{}MB", real_load, temp, total - available, total);
	}
}

fn extract_stat() -> u64 {
	let stat = fs::read_to_string("/proc/stat").unwrap();
	let cpu = stat.lines().find(|x| x.starts_with("cpu ")).unwrap();
	let times = cpu.split(' ');
	times
		.skip(1)
		.filter(|x| !x.is_empty())
		.take(3)
		.map(|x| x.parse::<u64>().unwrap())
		.sum::<u64>()
}
