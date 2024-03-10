use clap::{command, Arg};

fn main() {
    let match_result = command!()
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
        .get_matches();

    print!("{:?}", match_result);
}
