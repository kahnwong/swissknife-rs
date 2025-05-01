use passwords::PasswordGenerator;

pub fn password() {
    let pg = PasswordGenerator {
        length: 32,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    println!("{}", pg.generate_one().unwrap());
}
