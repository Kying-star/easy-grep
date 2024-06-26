use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<() , Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_name)?;

    let results = if config.case_sensitive {
        search_case_insensitive(&config.query , &contents)
    } else {
        search(&config.query , &contents)
    };

    for line in results {
        println!("{:?}" , line);
    }

    println!("{:?}",contents);

    Ok(())
}
pub struct Config {
    query: String,
    file_name: String,
    case_sensitive: bool
}

impl Config {
     pub fn new(mut args: env::Args) -> Result<Config , &'static str> {

        if args.len() < 3 {
            return  Err("参数不足")
        }

         args.next();

        let query = match args.next() {
            Some(query) => {
                query
            },
            None => return Err("query 参数错误")

        };
        let file_name = match args.next() {
            Some(file_name) => {
                file_name
            },
            None => {
                return Err("file_name 参数错误")
            }
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query , file_name , case_sensitive})
    }
}

pub fn search<'a>(query: &str , contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str , contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "abc";
        let contents = "abc";
        assert_eq!(vec!["abc"] , search(&query ,&contents));
    }

    #[test]
    fn case_not_sensitive(){
        let query = "abc";
        let contents = "ABC";
        assert_eq!(vec!["ABC"] , search_case_insensitive(&query ,&contents));
    }
}