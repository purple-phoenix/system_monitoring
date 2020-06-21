use sysinfo::{ComponentExt, SystemExt, ProcessorExt};
use regex::Regex;
use std::ops::Index;
use std::process::Command;

fn main() {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let processors = system.get_processors();
    let num_processors = processors.len();
    let mut cpu_infos = vec![make_cpu_info(0.0, 0.0); num_processors];

// Then let's print the temperature of the different components:
    let cpu_core_regex = Regex::new(r"Core (\d+)").unwrap();
    for component in system.get_components() {
        let component_temp = &component.get_temperature();
        let label = &component.get_label();

        for capture in cpu_core_regex.captures_iter(label) {
            let index_of_core_number = 1; // 0 is full regex capture, one is first group
            let core_index = capture[index_of_core_number].parse::<usize>().unwrap();
            let mut cpu_info = cpu_infos[core_index];
            cpu_info.temp.clone_from(component_temp);
            cpu_infos[core_index] = cpu_info;
        }

    }
    let cpu_num_regex = Regex::new(r"cpu(\d+)").unwrap();
    for processor in system.get_processors() {
        let name = processor.get_name();
        let usage = &processor.get_cpu_usage();

        for capture in cpu_num_regex.captures_iter(name) {
            let index_of_cpu_num = 1;
            let core_index = capture[index_of_cpu_num].parse::<usize>().unwrap();
            let mut cpu_info = cpu_infos[core_index];
            cpu_info.usage.clone_from(usage);
            cpu_infos[core_index] = cpu_info
        }

    }

    for cpu_info in cpu_infos {
        println!("{:?}", cpu_info)
    }

    // GPU

    let output = Command::new("nvidia-smi")
        .output().expect("Failed to execute nvidia-smi");

    let gpu_output_regex = Regex::new(r"| (/d+)%   (/d+)C    ..    (/d)+W / (/d+)W |    (/d+)MiB /  (/d+)MiB |      0%      Default |").unwrap();
    let output_lines = String::from_utf8(output.stdout).unwrap().lines();
    for line in output_lines {
        for capture in gpu_output_regex.captures_iter(line) {
            let fan_percent_index = 1;
            let temp_index = 2;
            let wattage_index = 3;
            let max_wattage_index = 4;
            let vram_usage_index = 5;
            let max_vram_index = 6;
        }
    }


}

#[derive(Clone, Copy, Debug)]
struct CPUInfo {
    temp: f32,
    usage: f32
}

#[derive(Clone, Copy, Debug)]
struct GPUInfo {
    temp: f32,
    usage: f32
}


fn make_cpu_info(temp: f32, usage: f32) -> CPUInfo {
    return CPUInfo {temp, usage};
}

fn make_gpu_info(temp: f32, usage: f32) -> GPUInfo {
    return GPUInfo {temp, usage};
}