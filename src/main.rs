use dorf_name::Language;
fn main() -> std::io::Result<()> {
    let lang = Language::load().expect("unable to load language files");
    // println!("{:#?}", lang.word("STRIKE"));
    for _ in 1..10 {
        println!("{}", lang.npc_name("DWARF"));
    }
    Ok(())
}
