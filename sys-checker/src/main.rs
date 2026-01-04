use sysinfo::{System};
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "sys-tools")]
#[command(about = "A simple system monitor", long_about = None)]
struct Cli {
    #[arg(short, long)]
    item: String,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let mut sys = System::new_all();

    sys.refresh_all();

    match args.item.as_str() {
        "memory" => {
            println!("Total Memory: {} KB", sys.total_memory());
            println!("Used Memory: {} KB", sys.used_memory());
        }
        "cpu" => {
            for (i, cpu) in sys.cpus().iter().enumerate() {
                println!("Core {}: {:.2}%", i, cpu.cpu_usage());
            }
        }
        _ => {
            println!("I don't recognize that item! Try 'memory' or 'cpu'.");
        }
    }

    Ok(())
}
