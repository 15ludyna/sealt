pub fn validate_domain(domain_str: &str) -> bool {
    match url::Url::parse(domain_str) {
        Ok(_) => true,
        Err(_) => false
    }
}
