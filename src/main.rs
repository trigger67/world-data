use chrono::NaiveDate;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut hashmap_results: HashMap<String, i32> = HashMap::new();

    let files = get_files_from_data_dir().unwrap();

    for file in files {
        add_data_from_file(file, &mut hashmap_results).unwrap();
    }

    let mut results: Vec<MonthlyData> = Vec::new();

    for (key, value) in &hashmap_results {
        let data = MonthlyData {
            year: key[0..4].parse().unwrap(),
            month: key[4..6].parse().unwrap(),
            age_group: key[6..].parse().unwrap(),
            number_of_death: *value,
        };
        println!("{:?}", data);
        results.push(data);
    }

    Ok(())
}

fn get_files_from_data_dir() -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_list: Vec<String> = Vec::new();

    let files = std::fs::read_dir("data").unwrap();

    for file in files {
        file_list.push(file.unwrap().path().into_os_string().into_string().unwrap());
    }

    file_list.sort();

    Ok(file_list)
}

#[derive(Debug)]
struct MonthlyData {
    year: i32,
    month: i32,
    age_group: usize,
    number_of_death: i32,
}

fn add_data_from_file(
    filename: String,
    hashmap_results: &mut HashMap<String, i32>,
) -> Result<&mut HashMap<String, i32>, Box<dyn Error>> {
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    for (_idx, line) in buffered.lines().enumerate() {
        let new_line = line?;

        let mut birth_date = String::from(&new_line[81..89]);
        birth_date = fix_date(birth_date);

        let mut death_date;

        if &new_line[154..155] != " " {
            death_date = String::from(&new_line[154..162]);
        } else if &new_line[155..156] != " " {
            death_date = String::from(&new_line[155..163]);
        } else if &new_line[156..157] != " " {
            death_date = String::from(&new_line[156..164]);
        } else {
            death_date = String::from(&new_line[157..168]);
        }

        death_date = fix_date(death_date);

        let birth_date = NaiveDate::parse_from_str(&birth_date, "%Y%m%d")?;
        let death_date = NaiveDate::parse_from_str(&death_date, "%Y%m%d")?;

        let days_lived: i32 = death_date.signed_duration_since(birth_date).num_days() as i32;
        let years_lived = days_lived / 365;
        let years_dozen = years_lived / 10;
        if years_dozen > 12 {
            println!("TOO OLD"); // Probably born on year 0000
            continue;
        } else if years_dozen < 0 {
            println!("TOO YOUNG"); // Probably born on year 0000
            continue;
        }

        let hashmap_key = format!("{}{}", death_date.format("%Y%m"), years_dozen);

        let count = hashmap_results.entry(hashmap_key).or_insert(0);
        *count += 1;
    }

    println!("HashMap result :{}", hashmap_results.len());

    Ok(hashmap_results)
}

fn fix_date(date: String) -> String {
    let mut new_date = String::from(&date[0..4]);

    if &date[4..6] == "00" {
        new_date.push_str("01");
    } else {
        new_date.push_str(&date[4..6]);
    }

    if &date[6..8] == "00" {
        new_date.push_str("01");
    } else {
        new_date.push_str(&date[6..8]);
    }

    new_date
}
