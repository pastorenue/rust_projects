use regex::Regex;

fn main() {
    // _test_regex()
    _another_regex_func()
}


/// .
///
/// ## Testing the Regex module
///
/// Searches for the first and last name and the year.
fn _test_regex() {
    let input_text = "Trevor|Noah|1989";
    let _capitalize = Regex::new(r"[A-Z]{1}[a-z]{2,}").unwrap(); 
    let my_pattern = r"(?<first_name>[A-Z]{1}[a-z]{2,8})\|(?<last_name>[A-Z]{1}[a-z]{2,8})\|(?<year>[0-9]{4})";
    let name_regex = Regex::new(my_pattern);

    let matcher = name_regex.unwrap().captures(input_text);
    if let Some(m) = matcher {
        println!("Formated output: {}-{}-{}",
            m.name("first_name").unwrap().as_str(),
            m.name("last_name").unwrap().as_str(),
            m.name("year").unwrap().as_str()
        );
    }
    // println!("{:?}", capitalize.find_iter("Treve Noah 1989"));
}

fn _another_regex_func() {
    let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
    let hay = "\
        path/to/foo:54:Blue Harvest..
        path/to/bar:90:Something, Something, Something, Dark Side
        path/to/baz:3:It's a Trap!
        ";
    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(hay).map(|c| c.extract()) {
        results.push((path.trim(), lineno.parse::<u64>().unwrap(), line));
    }

    println!("{:?}", results);
}
