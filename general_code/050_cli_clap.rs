// clap = "3.0.0-beta.5"
extern crate clap;

use clap::{Arg, App};

fn main() {
    let app = App::new("hello-clap");
    let iternum = Arg::with_name("n")
        .long("num")
        .takes_value(true)
        .help("You need to specify the max number of iterations.")
        .required(true);

   let funcatk = Arg::with_name("f")
        .long("func")
        .takes_value(true)
        .help("You need to specify the function to attack.")
        .required(true);

   let contractf = Arg::with_name("c")
        .long("contract")
        .takes_value(true)
        .help("You need to specify the contract file's name.")
        .required(true);

    let app = app.arg(iternum);
    let app = app.arg(funcatk);
    let app = app.arg(contractf);

    let matches = app.get_matches();

    let iternumout = matches.value_of("iternum")
        .expect("Required option.");
    let funcatkout = matches.value_of("funcatk")
        .expect("Required option.");
    let contractfout = matches.value_of("contractf")
        .expect("Required option.");

    println!("Your options are: {} {} {}!", iternumout, funcatkout, contractfout);
}
