use dorf_name::Language;
fn main() -> std::io::Result<()> {
    let lang = Language::load().expect("unable to load language files");
    // println!("{:?}", lang.word("ALE"));
    println!("{}", lang.npc_name());
    Ok(())
}
