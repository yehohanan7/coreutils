use std::env;

#[path="src/cli.rs"]
mod cli;

fn main() {
    let mut app = cli::create_app();

    let out_dir = match env::var("OUT_DIR") {
        Ok(dir) => dir,
        _ => return,
    };

    app.gen_completions("template", Shell::Zsh, out_dir.clone());
    app.gen_completions("template", Shell::Fish, out_dir.clone());
    app.gen_completions("template", Shell::Bash, out_dir.clone());
    app.gen_completions("template", Shell::PowerShell, out_dir.clone());
    app.gen_completions("template", Shell::Elvish, out_dir);
}
