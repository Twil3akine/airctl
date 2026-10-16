mod remote;

use clap::{Parser, Subcommand};
use std::time::{SystemTime, UNIX_EPOCH};

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
    Status {
        job_id: Option<String>,
    },

    Run {
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<String>,
    },

    Logs {
        job_id: String,
    },
}

fn new_job_id() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_millis();

    format!("{millis:x}-{:x}", std::process::id())
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status { job_id } => {
            if let Some(job_id) = job_id {
                let script = format!(
                    r#"job_id={}
                    job_dir="${{XDG_STATE_HOME:-$HOME/.local/state}}/airctl/jobs/$job_id"

                    if [ ! -d "$job_dir" ]; then
                        printf 'not-found\n'
                        exit 0
                    fi

                    if [ -f "$job_dir/exit-code" ]; then
                        code=$(cat "$job_dir/exit-code")

                        if [ "$code" -eq 0 ]; then
                            printf 'completed\n'
                        else
                            printf 'failed (%s)\n' "$code"
                        fi

                        exit 0

                    fi

                    if [ -f "$job_dir/pid" ]; then
                        pid=$(cat "$job_dir/pid")

                        if kill -0 "$pid" 2>/dev/null; then
                            printf 'running\n'
                        else
                            printf 'unknown\n'
                        fi

                        exit 0
                    fi

                    printf 'unknown\n'
                    "#,
                    shell_quote(&job_id),
                );

                match remote::run_script(&cli.host, &script) {
                    Ok(status) if status.success() => {}
                    Ok(status) => {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                    Err(err) => {
                        eprintln!("failed to execute remote script: {err}");
                        std::process::exit(1);
                    }
                }
            } else {
                let status = remote::check_host(&cli.host);
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
        }

        Commands::Run { command } => {
            let job_id = new_job_id();

            let args = command
                .iter()
                .map(|arg| shell_quote(arg))
                .collect::<Vec<_>>()
                .join(" ");

            let script = format!(
                r#"set -eu

                job_id={}
                job_dir="${{XDG_STATE_HOME:-$HOME/.local/state}}/airctl/jobs/$job_id"

                mkdir -p "$job_dir"

                nohup sh -c '
                job_dir=$1
                shift

                "$@" >"$job_dir/stdout" 2>"$job_dir/stderr"
                code=$?

                printf "%s\n" "$code" >"$job_dir/exit-code"
                ' sh "$job_dir" {} </dev/null >/dev/null 2>&1 &

                printf "%s\n" "$!" >"$job_dir/pid"
                "#,
                shell_quote(&job_id),
                args,
            );

            match remote::run_script(&cli.host, &script) {
                Ok(status) if status.success() => {
                    println!("job {job_id} started on {}", cli.host);
                }
                Ok(status) => {
                    eprintln!("failed to start remote job: {status}");
                    std::process::exit(1);
                }
                Err(err) => {
                    eprintln!("failed to execute remote script: {err}");
                    std::process::exit(1);
                }
            }
        }

        Commands::Logs { job_id } => {
            let script = format!(
                r#"job_id={}

                job_dir="${{XDG_STATE_HOME:-$HOME/.local/state}}/airctl/jobs/$job_id"

                if [ ! -d "$job_dir" ]; then
                    printf 'job not found: %s\n' "$job_id" >&2
                    exit 1
                fi

                if [ -f  "$job_dir/stdout" ]; then
                    cat "$job_dir/stdout"
                fi

                if [ -s "$job_dir/stderr" ]; then
                    printf '\n--- stderr ---\n' >&2
                    cat "$job_dir/stderr" >&2
                fi
                "#,
                shell_quote(&job_id),
            );

            match remote::run_script(&cli.host, &script) {
                Ok(status) if status.success() => {}
                Ok(status) => {
                    std::process::exit(status.code().unwrap_or(1));
                }
                Err(err) => {
                    eprintln!("failed to execute remote script: {err}");
                    std::process::exit(1);
                }
            }
        }
    }
}
