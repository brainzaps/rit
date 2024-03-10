use clap::{command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
        .about("A basic example")
        .subcommand(
            Command::new("register-person")
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
                ),
        )
        .subcommand(
            Command::new("register-pet").arg(
                Arg::new("pet-name")
                    .long("pet-name")
                    .short('n')
                    .required(true),
            ),
        )
        // .group(
        //     ArgGroup::new("person-register")
        //         .arg("firstname")
        //         .arg("lastname"),
        // )
        // .group(ArgGroup::new("pet-register").arg("pet-name"))
        .get_matches();

    // println!("{}", match_result.get_one::<bool>("fluffy").unwrap());

    let pet_args = match_result.subcommand_matches("register-pet");
    pet_args.unwrap().contains_id("pet-name");

    let pet_name = pet_args.unwrap().get_one::<String>("pet-name");

    print!("Pet name is: {}", pet_name.unwrap());
}
