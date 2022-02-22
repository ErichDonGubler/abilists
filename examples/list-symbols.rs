//! A Rust rewrite of [`list_symbols.zig`] that attempts to keep the exact same output format on
//! the happy path.
//!
//! [`list_symbols.zig`]: ../deps/glibc-abi-tool/list_symbols.zig

use abilists::{AbiList, GlibcVersion};
use clap::Parser;
use color_eyre::eyre::WrapErr;
use format::lazy_format;
use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(parse(from_os_str))]
    file_path: PathBuf,
}

struct GlibcLibraryName<'a>(pub &'a str);

impl<'a> Display for GlibcLibraryName<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(name) = self;
        write!(f, "lib{name}.so")
    }
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let Args { file_path } = Args::parse();

    let mut file = {
        let file = File::open(&file_path).wrap_err("failed to open file path")?;
        BufReader::new(file)
    };

    let abi_list = AbiList::from_reader(&mut file)
        .wrap_err("failed to read `abilist` structure from given file path")?;

    println!("Libraries:");
    abi_list
        .libraries()
        .iter()
        .enumerate()
        .for_each(|(idx, name)| {
            println!(" {idx} {}", GlibcLibraryName(&name));
        });

    println!("Versions:");
    let version_display = |version| {
        lazy_format!(move |f| {
            let GlibcVersion {
                major,
                minor,
                patch,
            } = &version;
            match patch {
                0 => write!(f, "{major}.{minor}"),
                patch => write!(f, "{major}.{minor}.{patch}"),
            }
        })
    };
    abi_list
        .versions()
        .iter()
        .enumerate()
        .for_each(|(idx, version)| {
            println!(" {idx} GLIBC_{}", version_display(version.clone()));
        });

    println!("Targets:");
    abi_list
        .targets()
        .iter()
        .enumerate()
        .for_each(|(idx, target)| {
            println!(" {idx} {target}");
        });

    println!("Functions:");
    abi_list.functions().iter().for_each(|func| {
        func.inclusions().for_each(|inclusion| {
            println!(" {}:", func.symbol_name());
            println!("  library: {}", GlibcLibraryName(inclusion.library()));

            print!("  versions:");
            inclusion.versions().iter().for_each(|version| {
                print!(" {}", version_display(version.clone()));
            });
            println!();

            print!("  targets:");
            inclusion.targets().for_each(|target| {
                print!(" {target}");
            });
            println!();
        })
    });

    println!("Objects:");
    abi_list.objects().iter().for_each(|obj| {
        obj.inclusions().for_each(|inclusion| {
            println!(" {}:", obj.symbol_name());
            println!("  size: {}", inclusion.size());
            println!("  library: {}", GlibcLibraryName(inclusion.library()));

            print!("  versions:");
            inclusion.versions().iter().for_each(|version| {
                print!(" {}", version_display(version.clone()));
            });
            println!();

            print!("  targets:");
            inclusion.targets().for_each(|target| {
                print!(" {target}");
            });
            println!();
        })
    });

    Ok(())
}
