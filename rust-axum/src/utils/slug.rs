use regex::Regex;
use unicode_normalization::UnicodeNormalization;

pub fn generate_slug(text: &String) -> String {
    let pattern = Regex::new(r"/[\u0300-\u036f]").unwrap();
    let other_pattern = Regex::new(r"[^\w\s-]").unwrap();
    let slug = text
        .to_lowercase()
        .nfd()
        .map(|c| c.to_string())
        .filter(|string| !(pattern.is_match(&string) || other_pattern.is_match(&string)))
        .collect::<String>();
    slug.replace(" ", "-")
}
