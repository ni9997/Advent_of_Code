use std::fs;

#[derive(Debug)]
pub enum InputError {
    DownloadError,
    FilesystemError,
}

pub fn get_input(year: usize, day: usize) -> Result<String, InputError> {
    let path = format!("input/{}/day_{:02}.txt", year, day);
    let input = match fs::read_to_string(&path) {
        Ok(data) => Ok(data),
        Err(_) => {
            let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
            let session_key = match std::env::var("SESSION_TOKEN") {
                Ok(key) => key,
                Err(_) => return Err(InputError::DownloadError),
            };
            let client = reqwest::blocking::Client::new();
            let input = match client
                .get(url)
                .header("Cookie", format!("session={}", session_key))
                .send()
            {
                Ok(download) => match download.text() {
                    Ok(text) => {
                        fs::write(&path, &text.trim_end()).unwrap();
                        Ok(text.trim_end().to_string())
                    }
                    Err(_) => Err(InputError::DownloadError),
                },
                Err(_) => Err(InputError::DownloadError),
            };
            input
        }
    };
    input
}
