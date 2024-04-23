use std::fs;
use std::path::Path;

use clap::Parser;
use regex::Regex;

//TODO Add option to replace spaces with tabs and vise versa
#[derive(Debug, Parser)]
#[command(author = "Lk <pleasenospam@protonmail.com>")]
#[command(version = "v0.1.0")]
#[command(about = "Remove repetitive white space characters", long_about = None)]
struct Opt {
    /// An input string to condense
    data: Option<String>,

    /// Should read from file instead of stdin
    #[structopt(short, long)]
    file: Option<std::path::PathBuf>,

    /// Whether to consider all consecutive white spaces as duplicates
    #[structopt(short, long, default_value_t = false)]
    aggressive: bool,

    /// aggressivly condense input to one line with only single spaces
    #[structopt(long, default_value_t = false)]
    one: bool,

    /// If sent a file file, edit in place
    #[structopt(short, long, default_value_t = false)]
    inplace: bool,

    /// Minimum number of consecutive white space characters to concense
    #[structopt(short, long, default_value_t = 2)]
    min: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Opt::parse();

    match args.file {
        Some(f) => return process_file(f, args.inplace, args.one),
        None => match args.data {
            Some(d) => {
                if args.one {
                    make_one(&d);
                    return Ok(());
                }
                match condense(&d) {
                    Ok(v) => {
                        println!("{}", v);
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => return Err("You must provide a pattern or a file".into()),
        },
    }
}

fn process_file(
    name: std::path::PathBuf,
    inplace: bool,
    one: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !is_valid_path_format(&name) {
        return Err("Was not given a valid file path".into());
    }
    let contents = fs::read_to_string(&name).expect("Should have been able to read the file");

    let v: String;
    if one {
        v = make_one(&contents);
    } else {
        v = condense(&contents)?;
    }


    if inplace {
        fs::write(name, v)?;
    } else {
        println!("{}", v);
    }

    Ok(())
}

fn is_valid_path_format(path_str: &std::path::PathBuf) -> bool {
    let path = Path::new(path_str);
    path.exists() && path.is_file()
}

//TODO: Check minimum number cli
//TODO: Add aggresive mode
//TODO: Check if they want all tabs swapped for spaces or vice versa
//TODO: What about mixed white space ( \t  \t), white space at the end of a line?
fn condense(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let space_re = match Regex::new(r" {2,}") {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let tab_re = match Regex::new(r"\t{2,}") {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let line_end_re = match Regex::new(r"[\r\n]{3,}") {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let leading_space = Regex::new(r"(?m)^ (\S)").unwrap();
    let trailing_space = Regex::new(r"(?m)(\S) $").unwrap();

    let leading_lines = Regex::new(r"^\s+(\S)").unwrap();
    let trailing_lines = Regex::new(r"[\n\r]{2,}$").unwrap();

    //TODO: Avoid this atrocity
    let t0 = leading_lines.replace_all(input, "$1");
    let t1 = trailing_lines.replace_all(&t0, "");
    let t2 = space_re.replace_all(&t1, " ");
    let t3 = tab_re.replace_all(&t2, "\t");
    //TODO: if aggressive, change to 1 \n
    let t4 = line_end_re.replace_all(&t3, "\n\n");
    let t5 = leading_space.replace_all(&t4, "$1");
    let t6 = trailing_space.replace_all(&t5, "$1");

    Ok(t6.to_string())
}

fn make_one(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    input.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}


//TODO: Write tests
