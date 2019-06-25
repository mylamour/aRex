use aho_corasick::AhoCorasickBuilder;
use clap::{App, Arg, SubCommand};
use std::{
    fs::read_to_string,
    fs::File,
    io,
    io::{prelude::*, BufReader},
    path::Path,
};

// use read_process_memory::{copy_address, CopyAddress, Pid, TryIntoProcessHandle};

// fn read_some_memory(pid: Pid, address: usize, size: usize) -> io::Result<()> {
//     let handle = pid.try_into_process_handle()?;
//     let _bytes = copy_address(address, size, &handle)?;
//     println!("Read {} bytes", size);
//     Ok(())
// }

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("unable to parse").to_lowercase())
        .collect()
}

fn acmatch(patterns: Vec<String>, target: String) -> Vec<String> {
    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(false)
        .build(patterns);

    let mut matches = vec![];

    for mat in ac.find_iter(&target) {
        matches.push(format!(
            "{{ ruleid: {}, start: {}, end: {} }}",
            mat.pattern(),
            mat.start(),
            mat.end()
        ));
    }

    if matches.is_empty() {
        println!("Not found it on your rules");
    } else {
        println!("【Matched】: {:?}", matches);
    }
    matches
}

fn main() {
    let cli = App::new("Pattern Mattch")
        .version("0.1")
        .author("Mour")
        .about("M-N Match")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET")
                .help("target")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("rule")
                .short("r")
                .long("rule")
                .value_name("RULE")
                .help("rule filename")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pid")
                .short("p")
                .long("pid")
                .value_name("PID")
                .help("process id")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let patterns = lines_from_file(cli.value_of("rule").unwrap());

    // let patterns = lines_from_file("webshell.list");
    let target = cli.value_of("target").unwrap();
    // let pid = cli.value_of("pid").unwrap();
    // read_some_memory(24, 0x100000, 100).unwrap();

    if Path::new(target).exists() {
        let content = read_to_string(target)
            .expect("Something went wrong reading the file")
            .to_ascii_lowercase();
        acmatch(patterns, content);
    } else {
        acmatch(patterns, target.to_string());
    }

}
