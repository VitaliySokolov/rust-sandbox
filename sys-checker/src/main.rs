use sysinfo::{System};
use clap::Parser;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "sys-tools")]
#[command(about = "A simple system monitor", long_about = None)]
struct Cli {
    #[arg(short, long)]
    item: String,
}

struct CpuDisplay;

impl CpuDisplay {
    fn format_usage(usage: f32) -> String {
        if usage > 80.0 {
            format!("High Usage: {:.1}%", usage)
        } else {
            format!("Normal: {:.1}%", usage)
        }
    }
}

fn print_stats(system: &sysinfo::System) {
    println!("Used Memory: {} KB", system.used_memory());
}


fn main() {
    let sys = Arc::new(Mutex::new(System::new_all()));
    let background_sys = Arc::clone(&sys);

    thread::spawn(move || {
        loop {
            if let Ok(mut s) = background_sys.lock() {
                s.refresh_all();
                println!("(Background: Data Refreshed)");
            }
            thread::sleep(Duration::from_secs(5));
        }
    });

    loop {
        println!("Press Enter to see current memory usage (or Ctrl+C to quit)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Lock to read
        if let Ok(s) = sys.lock() {
            println!("Memory Used: {} KB", s.used_memory());
        }
    }

    // loop {
    //     println!("{}[2J{}[1;1H", 27 as char, 27 as char);
    //
    //     sys.refresh_all();
    //
    //
    //     println!("--- System Monitor (Press Ctrl+C to quit) ---");
    //     print_stats(&sys);
    //
    //     for cpu in sys.cpus() {
    //         // println!("Core {}: {:.2}%", i, cpu.cpu_usage());
    //         let status = CpuDisplay::format_usage(cpu.cpu_usage());
    //         println!("{}", status);
    //     }
    //
    //     io::stdout().flush()?;
    //
    //     thread::sleep(Duration::from_secs(2));
    // }

    // let args = Cli::parse();
    // match args.item.as_str() {
    //     "memory" => {
    //         println!("Total Memory: {} KB", sys.total_memory());
    //         println!("Used Memory: {} KB", sys.used_memory());
    //     }
    //     "cpu" => {
    //         for (i, cpu) in sys.cpus().iter().enumerate() {
    //             println!("Core {}: {:.2}%", i, cpu.cpu_usage());
    //         }
    //     }
    //     _ => {
    //         println!("I don't recognize that item! Try 'memory' or 'cpu'.");
    //     }
    // }

    // Ok(())
}
