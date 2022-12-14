use dc_ock::safe_eval_with_stack;
use std::io::Write;
use std::{
    collections::VecDeque,
    io::{self, stdin, stdout},
    process::exit,
};

fn main() {
    let config = get_config();

    let prompt: String = {
        let this = config.get("prompt");
        match this {
            Some(x) => x,
            None => ">>> ",
        }
    }
    .to_string();

    let mut stk: VecDeque<f64> = VecDeque::new();

    loop {
        print!("{}", prompt);
        io::Write::flush(&mut stdout()).expect("Flush error");
        let mut in_str = String::new();

        match stdin().read_line(&mut in_str) {
            Ok(0) => exit(0),
            Ok(_) => (),
            Err(_) => panic!("Read error"),
        }

        match safe_eval_with_stack(in_str.trim(), stk.clone()) {
            Ok(x) => stk = x,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        }
    }
}

/// `get_config` returns a config object of type HashMap<String, String>,
/// containing the values of the config file.
///
fn get_config() -> std::collections::HashMap<String, String> {
    let binding = directories::ProjectDirs::from("com", "lc332", "dc_rs").unwrap();
    let config_file = format!(
        "{}{}",
        binding.config_dir().to_str().unwrap_or("."),
        "/dc_rs.toml"
    );

    // Create the config directory & file if they don't exist already.
    let _ = std::fs::create_dir_all(binding.config_dir());
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&config_file);

    // If the file WAS created, then we need to write the default config to it.
    if file.is_ok() {
        writeln!(file.as_ref().unwrap(), "prompt = \">>> \"").unwrap();
        std::mem::drop(file);
    } else {
        std::mem::drop(file)
    }

    // Finally, create the config object and return a hashmap of it.
    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_file))
        .add_source(config::Environment::with_prefix("DC_CONF"))
        .build()
        .unwrap();

    config
        .try_deserialize::<std::collections::HashMap<String, String>>()
        .unwrap()
}
