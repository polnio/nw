use clap::CommandFactory as _;
use clap_complete::Shell;
use std::path::PathBuf;

include!("src/utils/args.rs");

fn main() -> Result<(), std::io::Error> {
    println!("cargo:rerun-if-env-changed=GEN_ARTIFACTS");
    let outdir = match std::env::var_os("GEN_ARTIFACTS") {
        Some(outdir) => outdir,
        None => return Ok(()),
    };
    std::fs::create_dir_all(&outdir)?;
    let mut cmd = Args::command();
    let name = cmd.get_name().to_owned();
    for &shell in &[Shell::Bash, Shell::Zsh, Shell::Fish] {
        clap_complete::generate_to(shell, &mut cmd, &name, &outdir)?;
    }
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(PathBuf::from(outdir).join(format!("{name}.1")), buffer)?;
    Ok(())
}
