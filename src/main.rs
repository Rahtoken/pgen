#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

use std::str::FromStr;

mod pgen_lib;
use pgen_lib::gen_pwd;

fn main() {
    let cli_config = load_yaml!("cli.yml");
    let cli_args = App::from_yaml(cli_config).get_matches();
    const DEFAULT_MAX_LENGTH: i32 = 128;
    const DEFAULT_LENGTH: i32 = 16;
    let max_length: i32 = get_cli_param_or_default(&cli_args, "max-length").unwrap();
    let length: i32 = get_cli_param_or_default(&cli_args, "length").unwrap();
    let password_length = match length {
        l if l < 0 => DEFAULT_LENGTH,
        l if l > max_length => DEFAULT_MAX_LENGTH,
        _ => length,
    };
    let password = gen_pwd(password_length as usize);
    print!("{}", password);
}

fn get_cli_param_or_default<T: FromStr>(cli: &ArgMatches, param: &str) -> Result<T, T::Err> {
    cli.value_of(param).unwrap_or_default().parse::<T>()
}
