use std::error::Error;

pub fn get_files_from_dir(dir_name: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_list: Vec<String> = Vec::new();

    let files = std::fs::read_dir(dir_name).unwrap();

    for file in files {
        file_list.push(file.unwrap().path().into_os_string().into_string().unwrap());
    }

    file_list.sort();

    Ok(file_list)
}
