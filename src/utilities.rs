use std::fs::File;
use std::io::Read;
use std::str::FromStr;

pub fn load_file(file: &str) -> Result<String, String> {
    match File::open(file) {
        Ok(mut file) => {
            let mut content = String::new();

            if let Err(err) = file.read_to_string(&mut content) {
                Err(err.to_string())
            } else {
                Ok(content)
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn parse_pretty_err<F>(s: &str) -> Result<F, String>
where
    F: FromStr,
    <F as FromStr>::Err: std::fmt::Display,
{
    s.parse().map_err(|e| format!("Error: {}: {}", s, e))
}
