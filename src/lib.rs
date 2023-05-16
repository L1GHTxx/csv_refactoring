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
			
			let patterns = vec!["(S)", "(P)"];
		
			for pattern in patterns {
				if name.ends_with(pattern) {
					let pattern_length = pattern.len();
					println!("В поле c именем \"{}\" удален постфикс - {}", &name, &pattern);
					name.truncate(name.len() - pattern_length);
				}
				else if name.ends_with(pattern.to_lowercase().as_str()) {
					let pattern_length = pattern.len();
					println!("В поле c именем \"{}\" удален постфикс - {} Замечание: постфикс должен быть записан в верхнем регистре: {}", &name, &pattern.to_lowercase(), &pattern);
					name.truncate(name.len() - pattern_length);					
				}
			}			

            fields[0] = name.trim_end();
			let fields = fields.iter().map(|&f| f.trim_matches('"').to_string()).collect::<Vec<String>>();
			vec_fields.push(fields);
        }
		let temp = vec_fields.remove(0);
		vec_fields.sort_by_key(|c| c[0].to_lowercase().to_owned());
		vec_fields.insert(0, temp);



		let mut writer = WriterBuilder::new()
		.delimiter(b',')
		.quote_style(csv::QuoteStyle::Always)
        .from_writer(File::create(format!("refactored_{}",path))?);

		vec_fields.iter().for_each(|c| {
			writer.write_record(c);
		});
    }
    Ok(())
}