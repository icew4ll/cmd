#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate duct;
extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, Arg};

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

// main cli app
fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("ice")
        .about("git cli")
        .arg(Arg::with_name("output").help("function to exec").index(1))
        .get_matches();

    if let Some(o) = matches.value_of("output") {
        println!("INPUT: {}", o);
        match o {
            "dot" => dot(),
            "cmd" => gitpush(r"/m/cmd".to_string()),
            "b" => build(),
            _ => println!("0"),
        }
    }
}

// functions
fn dot() {
    let list = vec_of_strings![
        "/.config/fish/config.fish",
        "/.config/nvim/init.vim",
        "/.config/alacritty/alacritty.yml"
    ];
    // println!("{:?}", list)
    let dotdir = String::from("/m/dot");
    for x in list {
        // println!("{}", x)
        let cmd = format!(
            "rsync -av {}{} {}{}",
            dotenv!("HOME"),
            x,
            dotenv!("HOME"),
            dotdir
        );
        println!("\nSYNCING:\n{}", cmd);
        cmd!("sh", "-c", cmd).run().unwrap();
    }
    gitpush(dotdir);
}

fn gitpush(dir: String) {
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let gitpush = format!(
        "cd {}{};git add -A;git commit -m \"{}\";git push",
        dotenv!("HOME"),
        dir,
        utc.to_string()
    );
    println!("\nGITPUSH COMMAND:\n{}", gitpush);
    cmd!("sh", "-c", gitpush).run().unwrap();
}

fn build() {
    let cmd = format!("cd {}/m/cmd;cargo build", dotenv!("HOME"));
    cmd!("sh", "-c", cmd).run().unwrap();
}
