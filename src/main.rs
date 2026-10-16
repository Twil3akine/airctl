use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "airctl")]
#[command(version, about = "Control remote workloads from Air")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,

    Run {
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<String>,
    },

    Logs {
        job_id: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            println!("pro: unknown");
        }

        Commands::Run { command } => {
            println!("run: {}", command.join(" "));
        }

        Commands::Logs { job_id } => {
            println!("logs: {job_id}");
        }
    }
}
