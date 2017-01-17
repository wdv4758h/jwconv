#[macro_use]
extern crate clap;
extern crate jwconv;


fn main() {

    ////////////////////
    // Parse Arguments
    ////////////////////

    let yml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yml).get_matches();


    ////////////////////
    // Convert
    ////////////////////

    let data = matches.value_of("data").unwrap();
    let result = match matches.value_of("mode") {
        Some("r2h") => jwconv::romaji_to_hiragana(data),
        _ => unreachable!(),
    };

    println!("{}", result);
}
