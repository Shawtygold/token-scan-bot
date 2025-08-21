use regex::Regex;

pub fn extract_sol_token_address(msg: &str) -> Option<String> {
    let re = Regex::new("[1-9A-HJ-NP-Za-km-z]{32,44}").unwrap();

    if let Some(caps) = re.captures(msg) {
        return Some(caps[0].to_string());
    }

    None
}

pub fn extract_token_symbol(msg: &str) -> Option<String> {
    let re = Regex::new("\\$[a-zA-Z]+").unwrap();

    if let Some(caps) = re.captures(msg) {
        return Some(caps[0].to_string());
    }

    None
}
