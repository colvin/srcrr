use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use clap::{App, Arg};

fn main() {
    let args = App::new("srcrr")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .version(env!("CARGO_PKG_VERSION"))
        .about("load a development cli context")
        .arg(
            Arg::with_name("list")
                .long("list")
                .short("l")
                .help("List all known projects and exit"),
        )
        .arg(
            Arg::with_name("dirs")
                .long("dirs")
                .short("d")
                .help("Display (valid) search directories and exit"),
        )
        .arg(
            Arg::with_name("find")
                .value_name("PROJECT-NAME")
                .help("Name of the project to load"),
        )
        .get_matches();

    let locations: Vec<String> = env::var("SRCPATH")
        .unwrap()
        .split(":")
        .filter(|s| fs::metadata(s).is_ok())
        .map(|s| s.to_owned())
        .collect();

    if args.is_present("dirs") {
        for loc in locations {
            println!("{}", loc);
        }
        std::process::exit(0);
    }

    if args.is_present("list") {
        let mut projs = load_all_projects(&locations).unwrap();
        projs.sort();
        for path in projs {
            println!("{}", path.to_string_lossy());
        }
        std::process::exit(0);
    }

    match find(&locations, args.value_of("find").unwrap().to_owned()).unwrap() {
        Some(dir) => emit_shell(dir),
        None => {}
    }
}

fn load_all_projects(locs: &Vec<String>) -> io::Result<Vec<PathBuf>> {
    let mut all = Vec::new();
    for loc in locs {
        let mut found = walk_dir(PathBuf::from(&loc), None)?;
        all.append(&mut found);
    }
    return Ok(all);
}

fn find(locs: &Vec<String>, name: String) -> io::Result<Option<PathBuf>> {
    for loc in locs {
        let mut found = walk_dir(PathBuf::from(&loc), Some(PathBuf::from(&name)))?;
        if found.len() > 0 {
            return Ok(Some(found.remove(0)));
        }
    }
    Ok(None)
}

fn walk_dir(dir: PathBuf, find: Option<PathBuf>) -> io::Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    for dent in fs::read_dir(dir)? {
        if let Ok(d) = dent {
            if let Ok(m) = d.metadata() {
                if m.is_dir() {
                    if let Some(name) = &find {
                        if d.path().file_name().unwrap() == name {
                            found.push(d.path());
                            return Ok(found);
                        }
                    } else {
                        found.push(d.path());
                    }
                }
            } // TODO: else print warning
        } // TODO: else print warning
    }
    Ok(found)
}

fn emit_shell(dir: PathBuf) {
    println!(
        r##"cd {};
for i in  /etc/srcrrrc $HOME/.srcrrrc .srcrrrc ;
do
	test -f $i && . $i ;
done
"##,
        dir.to_string_lossy()
    );
}
