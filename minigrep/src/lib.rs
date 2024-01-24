use std::env;
use std::error::Error;
use std::fs;

/// Hold the configuration for the search
///
/// # Examples
///
/// ```
/// // A config to find the word "use" in the file README.md in
/// // a case-insensitive manner
/// use minigrep::Config;
///
/// let conf = Config {
///     String::from("use"),
///     String::from("README.md"),
///     true
/// };
/// ```
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Build a Config object
    ///
    /// # Examples
    ///
    /// ```
    /// use mingrep::Config
    /// // normally this would come in via the command line with env::args()
    ///
    /// let args = vec![String::from("use"),
    ///   String::from("README.md"),
    ///   String::from("--ignore")
    /// ];
    ///
    /// let cfg = Config::build(args.iter()).unwrap_or_else(|err|{
    ///     errprintln!("Problems")
    /// });
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip the program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string needed"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path needed"),
        };

        let ignore_case = match args.next() {
            Some(arg) => !arg.is_empty(),
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file_path)?;

    let results = if cfg.ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };
    let mut i: u32 = 0;
    for line in results {
        i += 1;
        println!("match {i}: {line}");
    }

    Ok(())
}

/// Search contents of a file for a given query (case sensitive)
///
/// # Example
///
/// ```
/// use mingrep::search;
///
/// let contents = String::from("Were I to weep\nI would weep for me\nnot for thee");
/// let query = String::from("We");
/// println("{:?}", search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Search contents of a file for a given query (case insensitive)
///
/// # Example
///
/// ```
/// use mingrep::search_case_insensitive;
///
/// let contents = String::from("Were I to weep\nI would weep for me\nnot for thee");
/// let query = String::from("We");
/// println("{:?}", search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
