use crate::utils::{args::ARGS, config::CONFIG};
use subprocess::Exec;

pub fn append_args(mut command: Exec) -> Exec {
    if ARGS.offline {
        command = command.args(&["--offline", "--no-net"])
    }
    if ARGS.quiet {
        command = command.arg("--quiet");
    }
    if let Some(max_tasks) = CONFIG.general().max_tasks() {
        command = command.args(&["--max-jobs", &max_tasks.to_string()]);
    }
    if let Some(max_cores) = CONFIG.general().max_cores() {
        command = command.args(&["--cores", &max_cores.to_string()]);
    }
    return command;
}
