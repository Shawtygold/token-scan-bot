use anyhow::Error;
use regex::Regex;

pub fn extract_sol_token_address(msg: &str) -> Result<Option<String>, Error> {
    let re = Regex::new("[1-9A-HJ-NP-Za-km-z]{32,44}")?;

    if let Some(caps) = re.captures(msg) {
        return Ok(Some(caps[0].to_string()));
    }

    Ok(None)
}

pub fn extract_token_symbol(msg: &str) -> Result<Option<String>, Error> {
    let re = Regex::new("\\$[a-zA-Z]+")?;

    if let Some(caps) = re.captures(msg) {
        return Ok(Some(caps[0].to_string()));
    }

    Ok(None)
}
