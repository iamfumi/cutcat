use clap_complete::Shell;
use clap::{IntoApp, Command};
use std::path::Path;
use std::fs::File;

include!("src/cli.rs");

fn generate(s: Shell, app: &mut Command, outdir: &Path, file: &str) {
    let destfile = outdir.join(file);
    println!("dest: {}", destfile.display());
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();

    clap_complete::generate(s, app, "cutcat", &mut dest);
}

fn main() {
    let mut app = Options::command();
    app.set_bin_name("cutcat");

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");

    generate(Shell::Bash, &mut app, &outdir, "bash/cutcat");
    generate(Shell::Elvish, &mut app, &outdir, "elvish/cutcat");
    generate(Shell::Fish, &mut app, &outdir, "fish/cutcat");
    generate(Shell::PowerShell, &mut app, &outdir, "powershell/cutcat");
    generate(Shell::Zsh, &mut app, &outdir, "zsh/_cutcat");
}
