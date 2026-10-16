use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "airctl")]
#[command(version, about = "Control remote workloads from Air")]
struct Cli {
    #[arg(short = 'H', long, default_value = "pro", global = true)]
    host: String,

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
            println!("{}: unknown", cli.host);
        }

        Commands::Run { command } => {
            println!("host: {}", cli.host);
            println!("run: {}", command.join(" "));
        }

        Commands::Logs { job_id } => {
            println!("host: {}", cli.host);
            println!("logs: {job_id}");
        }
    }
}
