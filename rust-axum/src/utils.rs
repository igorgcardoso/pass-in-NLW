use regex::Regex;
use unicode_normalization::UnicodeNormalization;

pub fn generate_slug(text: &String) -> String {
    let pattern = Regex::new(r"/[\u0300-\u036f]").unwrap();
    let mut slug = text.nfd().collect::<String>();
    slug = pattern.replace_all(&slug, "").to_string();
    slug = slug.to_lowercase();
    let other_pattern = Regex::new(r"[^\w\s-]").unwrap();
    slug = other_pattern.replace_all(&slug, "").to_string();
    slug.replace(" ", "-")
}
