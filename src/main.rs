use clap::Parser;
use dorf_name::Language;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Generate character names from DF language files
struct Args {
    /// How many names to generate
    #[arg(short, long, value_name = "INT")]
    count: Option<u8>,

    /// Dump a word structure
    #[arg(short, long, value_name = "STR")]
    word: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Args::parse();
    let lang = Language::load().expect("unable to load language files");

    if let Some(count) = cli.count {
        for _ in 1..count {
            println!("{}", lang.npc_name("DWARF"));
        }
    } else if let Some(word) = cli.word {
        println!("{:#?}", lang.word(&word.to_uppercase()));
    } else {
        println!("{}", lang.npc_name("DWARF"));
    }
    Ok(())
}
