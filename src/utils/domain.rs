pub fn validate_domain(domain: &str) -> bool {
    let regex = regex::Regex::new(r"^(http|https)://(www\.)?[a-z0-9]+(\.[a-z]+)+(:[0-9]+)?(/.*)?$").unwrap();
    regex.is_match(domain)
}
