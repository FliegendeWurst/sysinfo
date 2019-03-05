use std::fs;

fn main() {
    let load = fs::read_to_string("/proc/loadavg").unwrap().split(' ').nth(0).unwrap().parse::<f64>().unwrap();
    let num_cpus = fs::read_dir("/sys/devices/system/cpu").unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.starts_with("cpu") && x.chars().nth(3).unwrap().is_numeric())
        .count() as f64;
    let temp = fs::read_to_string("/sys/class/hwmon/hwmon0/temp1_input").unwrap().trim().parse::<f64>().unwrap() / 1000.0;
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let mut meminfo = meminfo.split('\n');
    let total = meminfo.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u64>().unwrap() / 1_000;
    let available = meminfo.nth(1).unwrap().split_whitespace().nth(1).unwrap().parse::<u64>().unwrap() / 1_000;
    println!("{:.1}% {:.1}°C {}/{}MB", load / num_cpus * 100.0, temp, total - available, total);
}
