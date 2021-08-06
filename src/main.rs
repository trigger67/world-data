mod drawing;
mod report_generation;
mod tools;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const START_YEAR: i32 = 2016;
const END_YEAR: i32 = 2021;
const NUMBER_OF_YEARS: i32 = END_YEAR - START_YEAR + 1;
const NUMBER_OF_YEARS_USIZE: usize = NUMBER_OF_YEARS as usize;
const REPORT1_NAME: &str = "1_death_full_year";
const REPORT2_NAME: &str = "2_death_part_year";
const NB_MONTH_FOR_REPORT_2: i32 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    // Extract data from files
    let mut hashmap_results: HashMap<String, i32> = HashMap::new();

    let files = tools::get_files_from_dir(String::from("data/deaths")).unwrap();

    for file in files {
        compute_data_from_file(file, &mut hashmap_results).unwrap();
    }

    let results = transform_data_into_vector(&hashmap_results);

    draw_png_for_first_report(&results);
    draw_png_for_second_report(&results);

    report_generation::generate_all_reports();

    Ok(())
}

// TODO
// 5 years only for first report, 6 for second
// Merge duplicated functions

fn draw_png_for_first_report(results: &Vec<MonthlyData>) {
    for age_group in (0usize..12usize).rev() {
        let age_group_full = format!("{}-{}", age_group * 10, age_group * 10 + 9);
        let chart_name = format!("{} ans", age_group_full);
        let mut file_name = String::new();
        if age_group > 9 {
            file_name.push('a');
        }
        file_name.push_str(&format!("{}.png", age_group_full));
        let computed_data = generate_data_for_age(age_group, &results);
        drawing::draw(computed_data, file_name, chart_name, REPORT1_NAME).unwrap();
    }
}

fn draw_png_for_second_report(results: &Vec<MonthlyData>) {
    for age_group in (0usize..12usize).rev() {
        let age_group_full = format!("{}-{}", age_group * 10, age_group * 10 + 9);
        let chart_name = format!("{} ans", age_group_full);
        let mut file_name = String::new();
        if age_group > 9 {
            file_name.push('a');
        }
        file_name.push_str(&format!("{}.png", age_group_full));
        let computed_data = generate_data_for_age_part_of_year(age_group, &results);
        drawing::draw(computed_data, file_name, chart_name, REPORT2_NAME).unwrap();
    }
}

fn transform_data_into_vector(hashmap_results: &HashMap<String, i32>) -> Vec<MonthlyData> {
    let mut results: Vec<MonthlyData> = Vec::new();

    for (key, value) in hashmap_results {
        let data = MonthlyData {
            year: key[0..4].parse().unwrap(),
            month: key[4..6].parse::<i32>().unwrap() - 1,
            age_group: key[6..].parse().unwrap(),
            number_of_death: *value,
        };
        //println!("{:?}", data);
        results.push(data);
    }

    return results;
}

fn generate_data_for_age(
    age_group: usize,
    results: &Vec<MonthlyData>,
) -> [i32; NUMBER_OF_YEARS_USIZE] {
    let mut computed_data: [i32; NUMBER_OF_YEARS_USIZE] = [0; NUMBER_OF_YEARS_USIZE];

    for year in START_YEAR..(END_YEAR + 1) {
        for data_element in results {
            if data_element.year == year {
                if data_element.age_group == age_group {
                    let number_death = data_element.number_of_death;
                    let _month = data_element.month;
                    computed_data[(year - START_YEAR) as usize] += number_death;
                }
            }
        }
    }

    computed_data
}

fn generate_data_for_age_part_of_year(
    age_group: usize,
    results: &Vec<MonthlyData>,
) -> [i32; NUMBER_OF_YEARS_USIZE] {
    let mut computed_data: [i32; NUMBER_OF_YEARS_USIZE] = [0; NUMBER_OF_YEARS_USIZE];

    for year in START_YEAR..(END_YEAR + 1) {
        for data_element in results {
            if data_element.year == year {
                if data_element.age_group == age_group {
                    if data_element.month <= NB_MONTH_FOR_REPORT_2 {
                        let number_death = data_element.number_of_death;
                        let _month = data_element.month;
                        computed_data[(year - START_YEAR) as usize] += number_death;
                    }
                }
            }
        }
    }

    computed_data
}

fn compute_data_from_file(
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
            //println!("TOO OLD"); // Probably born on year 0000
            continue;
        } else if years_dozen < 0 {
            //println!("TOO YOUNG"); // Probably born on year 0000
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

#[derive(Debug)]
struct MonthlyData {
    year: i32,
    month: i32,
    age_group: usize,
    number_of_death: i32,
}
