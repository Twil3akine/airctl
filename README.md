# airctl

A small CLI for running and managing remote workloads over SSH.

`airctl` is designed for using a lightweight machine as a control terminal while running workloads on a more powerful remote host.

## Features

- Check remote host connectivity
- Run commands as background jobs
- Check job status
- Read stdout and stderr
- List remote jobs
- Cancel running jobs
- Select the target SSH host

## Requirements

- Rust
- OpenSSH
- SSH access to the remote host
- A POSIX-compatible `sh` on the remote host

## Development

The repository includes a Nix flake.

```sh
nix develop
cargo build
```

Or, with an existing Rust environment:

```sh
cargo build
```

## Usage

```text
airctl [OPTIONS] <COMMAND>
```

The default remote host is `pro`.

You can select another SSH host with:

```sh
airctl --host <HOST> <COMMAND>
```

### Check host status

```sh
airctl status
```

Example:

```text
pro: online
```

### Run a command

```sh
airctl run <COMMAND>...
```

For example:

```sh
airctl run uname -a
airctl run python3 /tmp/script.py
airctl run cargo test
```

`airctl` starts the command as a background job on the remote host and returns a job ID.

```text
job 1a14697a771-3592 started on pro
```

### Check job status

```sh
airctl status <JOB_ID>
```

Possible states include:

```text
running
completed
failed (42)
cancelling
cancelled
unknown
not-found
```

### Read job logs

```sh
airctl logs <JOB_ID>
```

This prints the captured standard output and standard error of the job.

### List jobs

```sh
airctl jobs
```

Example:

```text
1a14688f60a-1ed7    completed
1a1468e65b9-252f    cancelled
1a14620079d-163dd   failed (42)
```

### Cancel a job

```sh
airctl cancel <JOB_ID>
```

## Remote job storage

Jobs are stored on the remote host under:

```text
${XDG_STATE_HOME:-$HOME/.local/state}/airctl/jobs/
```

Each job has its own directory containing runtime state such as logs, process IDs, and exit status.

## License

MIT
