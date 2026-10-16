use clap::{Parser, Subcommand};
use std::process::Command;

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
            let status = Command::new("ssh")
                .args([
                    "-o",
                    "BatchMode=yes",
                    "-o",
                    "ConnectTimeout=3",
                    &cli.host,
                    "true",
                ])
                .status();

            match status {
                Ok(status) if status.success() => {
                    println!("{}: online", cli.host);
                }
                Ok(_) => {
                    println!("{}: unreachable", cli.host);
                }

                Err(err) => {
                    eprintln!("failed to execute ssh: {err}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Run { command } => {
            let status = Command::new("ssh")
                .arg(&cli.host)
                .arg("--")
                .args(&command)
                .status();

            match status {
                Ok(status) => {
                    if let Some(code) = status.code() {
                        std::process::exit(code);
                    } else {
                        eprintln!("remote command terminated by signal");
                        std::process::exit(1);
                    }
                }
                Err(err) => {
                    eprintln!("failed to execute ssh: {err}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Logs { job_id } => {
            println!("host: {}", cli.host);
            println!("logs: {job_id}");
        }
    }
}
