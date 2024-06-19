import unicodedata as ud
import re

def generate_slug(title: str) -> str:
    slug = ud.normalize("NFD", title)
    slug = re.sub(r"[\u0300-\u036f]", "", slug)
    slug = slug.lower()
    slug = re.sub(r"[^\w\s-]", "", slug)
    slug = re.sub(r"[\s+]", "-", slug)
    return slug
