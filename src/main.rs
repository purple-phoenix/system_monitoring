use sysinfo::{ComponentExt, SystemExt, ProcessorExt};
use regex::Regex;
use std::ops::Index;
use std::process::Command;
use console::Term;

fn main() {

    loop {

        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        let processors = system.get_processors();
        let num_processors = processors.len();
        let num_gpus = 2;
        let mut cpu_infos = vec![make_cpu_info(0.0, 0.0); num_processors];
        let mut gpu_infos = vec![make_gpu_info(0.0, 0.0,
                                               0.0, 0.0, 0.0,
                                               0.0, 0.0); num_gpus];

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
        let term = Term::stdout();
        let mut terminal_lines = vec![];

        for (index, cpu_info) in cpu_infos.iter().enumerate() {
            let cpu_output = format!("CPU {} is {}C with {}% usage",
                                     index, cpu_info.temp, cpu_info.usage);
            terminal_lines.push(cpu_output.to_owned());
        }

        // GPU

        let output = Command::new("nvidia-smi")
            .output().expect("Failed to execute nvidia-smi");

        let gpu_output_regex = Regex::new(
            r"\| +(\d+)% +(\d+)C +.. +(\d)+W / (\d+)W \| +(\d+)MiB / +(\d+)MiB \| +0% +Default \|")
            .unwrap();
        let output = String::from_utf8(output.stdout).unwrap();
        let output_lines = output.lines();
        let mut gpu_index = 0;
        for line in output_lines {
            for capture in gpu_output_regex.captures_iter(line) {

                let fan_percent_index = 1;
                let temp_index = 2;
                let wattage_index = 3;
                let max_wattage_index = 4;
                let vram_usage_index = 5;
                let max_vram_index = 6;

                let fan_percent = &capture[fan_percent_index].parse::<f32>().unwrap();
                let temp = &capture[temp_index].parse::<f32>().unwrap();
                let wattage = &capture[wattage_index].parse::<f32>().unwrap();
                let max_wattage = &capture[max_wattage_index].parse::<f32>().unwrap();
                let vram_usage = &capture[vram_usage_index].parse::<f32>().unwrap();
                let max_vram = &capture[max_vram_index].parse::<f32>().unwrap();

                let mut gpu_info = gpu_infos[gpu_index];
                gpu_info.fan_percentgit .clone_from(fan_percent);
                gpu_info.temp.clone_from(temp);
                gpu_info.wattage.clone_from(wattage);
                gpu_info.max_wattage.clone_from(max_wattage);
                gpu_info.vram_usage.clone_from(vram_usage);
                gpu_info.max_vram.clone_from(max_vram);
                gpu_infos[gpu_index] = gpu_info;

                gpu_index += 1;
            }
        }


        for (index, gpu_info) in gpu_infos.iter().enumerate() {
            let gpu_output = format!("GPU {} is {}C pulling {}W / {}W\n\
            VRAM Usage {}MiB / {}MiB\nFans are at {}%", index, gpu_info.temp,
                                     gpu_info.wattage, gpu_info.max_wattage,
                                     gpu_info.vram_usage, gpu_info.max_vram, gpu_info.fan_percent);
            terminal_lines.push(gpu_output.to_owned());
        }
        term.clear_screen();
        for line in terminal_lines {
            term.write_line(line.as_str());
        }

        Command::new("sleep").arg("2").output().unwrap();

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
    usage: f32,
    fan_percent: f32,
    wattage: f32,
    max_wattage: f32,
    vram_usage: f32,
    max_vram: f32,
}


fn make_cpu_info(temp: f32, usage: f32) -> CPUInfo {
    return CPUInfo {temp, usage};
}

fn make_gpu_info(temp: f32, usage: f32, fan_percent: f32,
                 wattage: f32, max_wattage: f32, vram_usage: f32, max_vram:f32) -> GPUInfo {
    return GPUInfo {temp, usage, fan_percent, wattage, max_wattage, vram_usage, max_vram};
}
