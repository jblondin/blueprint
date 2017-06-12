#[macro_export]
macro_rules! blueprint {
    ($runner:ident) => {
        pub mod blueprint_common {
            use std::env;
            use std::path::Path;
            use etl::DataFrame;

            fn usage(exe_name: &str) {
                println!("Usage: {} <config file>", exe_name);
            }

            enum ReturnValue {
                Success,
                InvalidArgs,
                InvalidCall,
                FileNotFound,
                EtlError,
            }

            pub fn main() {
                let arg_count = env::args().count();
                if arg_count == 0 {
                    usage("<executable name>");
                    ::std::process::exit(ReturnValue::InvalidArgs as i32);
                } else if arg_count != 2 {
                    usage(&env::args().next().unwrap()[..]);
                    ::std::process::exit(ReturnValue::InvalidCall as i32);
                } else {
                    let mut args = env::args();
                    let exe_name = args.next().unwrap();
                    let config_file = args.next().unwrap();

                    let config_path = Path::new(&config_file[..]);
                    if !config_path.exists() {
                        println!("Specified config file {} doest not exist\n", config_file);
                        usage(&exe_name[..]);
                        ::std::process::exit(ReturnValue::FileNotFound as i32);
                    }

                    match DataFrame::load(&config_path) {
                        Ok((config, df)) => {
                            super::$runner(df, config);
                        }
                        Err(e) => {
                            println!("Error loading data file: {}", e);
                            ::std::process::exit(ReturnValue::EtlError as i32);
                        }
                    }
                    ::std::process::exit(ReturnValue::Success as i32);
                }
            }
        }
        fn main() {
            blueprint_common::main();
        }
    }
}
