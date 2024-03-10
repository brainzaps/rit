use clap::{command, Arg, ArgGroup};

fn main() {
    let match_result = command!()
        .about("A basic example")
        .group(
            ArgGroup::new("person-register")
                .arg("firstname")
                .arg("lastname"),
        )
        .arg(
            Arg::new("firstname")
                .short('f')
                .alias("fname")
                .required(true)
                .help("The person's first name")
                .conflicts_with("fluffy"),
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .alias("lname")
                .required(true)
                .help("The person's last name"),
        )
        .arg(Arg::new("fluffy").long("fluffy"))
        .group(ArgGroup::new("pet-register").arg("pet-name"))
        .arg(
            Arg::new("pet-name")
                .long("pet-name")
                .short('n')
                .required(true),
        )
        .get_matches();

    print!("{:?}", match_result);
}
