#![allow(unused)]
use csv::{WriterBuilder, QuoteStyle};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Config {
	forse: bool,
    file_paths: Vec<String>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        return Ok(Config {
			forse: args.all(|c| c == "-f"),
            file_paths: args.filter(|c| c != "-f").collect::<Vec<String>>(),
        });
    }
}



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	//let map: HashMap<String, String> = HashMap::new();

    for (i, path) in config.file_paths.iter().enumerate() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

		let mut vec_fields: Vec<Vec<String>> = Vec::new();

        // Читаем каждую строку таблицы исправляем первый столбец и записываем в новый файл
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let mut fields = line.split_terminator(',').collect::<Vec<_>>();
            
            let mut name = fields[0].trim_matches('"').to_string();

			match &name[(name.len() - 3)..] {
				"(S)" => {
					println!("в поле c именем \"{}\" удален постфикс", &name);
					name = name.replace("(S)", "").trim_end().to_string();
				},
				"(P)" => {
					println!("в поле c именем \"{}\" удален постфикс", &name);
					name = name.replace("(S)", "").trim_end().to_string();
				},
				"(p)" => {
					println!("В поле {} постфикс должен иметь верхний регистр (p) -> (P)", &name);
				},
				"(s)" => {
					println!("В поле {} постфикс должен иметь верхний регистр (s) -> (S)", &name);
				},
				_ => (),
			}


            fields[0] = name.as_str();
			let fields = fields.iter().map(|&f| f.trim_matches('"').to_string()).collect::<Vec<String>>();
			vec_fields.push(fields);
        }
		let temp = vec_fields.remove(0);
		vec_fields.sort_by_key(|c| c[0].to_lowercase().to_owned());
		vec_fields.insert(0, temp);



		let mut writer = WriterBuilder::new()
		.delimiter(b',')
		.quote_style(csv::QuoteStyle::Always)
        .from_writer(File::create(format!("refactored{}.csv", i + 1))?);

		vec_fields.iter().for_each(|c| {
			writer.write_record(c);
		});
    }
    Ok(())
}
