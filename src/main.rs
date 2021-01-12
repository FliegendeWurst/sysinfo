use std::fs;

const WAIT_MS: u32 = 500;

fn main() {
	let stat_1 = extract_stat();
	std::thread::sleep_ms(WAIT_MS);
	let stat_2 = extract_stat();
	/*
	let load = fs::read_to_string("/proc/loadavg").unwrap().split(' ').nth(0).unwrap().parse::<f64>().unwrap();
	let num_cpus = fs::read_dir("/sys/devices/system/cpu").unwrap()
		.map(|x| x.unwrap().file_name().into_string().unwrap())
		.filter(|x| x.starts_with("cpu") && x.chars().nth(3).unwrap().is_numeric())
		.count() as f64;
	*/
	let real_load = (stat_2 - stat_1) * 1000 / (WAIT_MS as u64);
	let temp = fs::read_to_string("/sys/class/hwmon/hwmon2/temp1_input").unwrap().trim().parse::<u64>().unwrap() / 1000;
	let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
	let mut meminfo = meminfo.split('\n');
	let total = meminfo.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u64>().unwrap() / 1_000;
	let available = meminfo.nth(1).unwrap().split_whitespace().nth(1).unwrap().parse::<u64>().unwrap() / 1_000;
	println!("{}% {:.1}°C {}/{}MB", real_load, temp, total - available, total);
}

fn extract_stat() -> u64 {
	let stat = fs::read_to_string("/proc/stat").unwrap();
	let cpu = stat.lines().find(|x| x.starts_with("cpu ")).unwrap();
	let times = cpu.split(' ');
	times.skip(1).filter(|x| !x.is_empty()).take(3).map(|x| x.parse::<u64>().unwrap()).sum::<u64>()
}
