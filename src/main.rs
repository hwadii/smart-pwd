#![allow(clippy::redundant_closure)]

use itertools::{Itertools, Position};
use dirs::home_dir;
use std::borrow::Cow;
use std::env;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

fn main() -> Result<()> {
    let pwd = env::current_dir()?;

    let mut ancestors = pwd.ancestors().collect::<Vec<_>>();
    ancestors.reverse();

    let home_pos = ancestors
        .iter()
        .position(|&i| i==home_dir().unwrap().as_path());

    if let Some(t) = home_pos {
        ancestors.drain(0..t);
        print!("~");
    }

    // println!("{:?}", ancestors);
    for pos in ancestors.iter().with_position() {
        // println!("{:?}", pos);
        match pos {
            Position::First(_)  => print!("/"),
            Position::Only(_) => {
                if !home_pos.is_some() {
                    print!("/");
                }
            }
            _ => {}
        }

        {
            let inner = pos.into_inner();

            if inner.parent().is_some() {
                match pos {
                    Position::Last(_) => {
                        print!("{}", path_file_name_to_string(inner).unwrap());
                    }
                    Position::Only(_) => {
                        if !home_pos.is_some() {
                            print!("{}", path_file_name_to_string(inner).unwrap());
                        }
                    }
                    Position::First(_) => {
                        if !home_pos.is_some() {
                            let name = shortest_unique_path_prefix(inner);
                            print!("{}", name);
                        }
                    }
                    _ => {
                        let name = shortest_unique_path_prefix(inner);
                        print!("{}", name);
                    }
                }
            }
        }

        match pos {
            Position::Middle(_) => print!("/"),
            _ => {}
        }
    }

    println!();

    Ok(())
}

fn shortest_unique_path_prefix(path: &Path) -> String {
    let name = path_file_name_to_string(path).unwrap();
    let contents = dirs_in(path, &name);
    shortest_unique_prefix(&name, &contents).to_string()
}

fn dirs_in(path: &Path, name: &str) -> Vec<String> {
    let mut contents = path
        .parent()
        .expect("no parent")
        .read_dir()
        .expect("read_dir failed")
        .map(|entry| entry.unwrap().path())
        .filter(|entry| entry.is_dir())
        .map(|entry| path_file_name_to_string(&entry).unwrap())
        .filter(|entry_name| entry_name != name)
        .collect::<Vec<_>>();
    contents.sort_unstable();
    contents
}

fn path_file_name_to_string(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

fn shortest_unique_prefix<'a, S: AsRef<str>>(name: &'a str, others: &[S]) -> Cow<'a, str> {
    for n in 1..name.len() {
        let sub = name.chars().take(n).collect::<String>();
        if others
            .iter()
            .find(|other| other.as_ref().starts_with(&sub))
            .is_none()
        {
            return Cow::Owned(sub);
        }
    }

    Cow::Borrowed(name)
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_something() {
        let name = "minor";
        let dirs = vec!["archive", "bin", "major", "reference"];

        assert_eq!("mi", shortest_unique_prefix(&name, &dirs));
    }

    #[test]
    fn test_one_dir_in_path() {
        let name = "minor";
        let dirs = Vec::<&str>::new();

        assert_eq!("m", shortest_unique_prefix(&name, &dirs));
    }

    #[test]
    fn test_accent_in_dir() {
        let name = "téléchargments";
        let dirs = vec!["téla", "téle"];

        assert_eq!("télé", shortest_unique_prefix(&name, &dirs));
    }
}
