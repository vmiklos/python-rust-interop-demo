#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Generate HTML with Rust.
//!
//! This is more or less a Rust port of the Python package, mostly because
//! <https://crates.io/crates/html-builder> would require you to manually escape attribute values.

/// Generates xml/html documents.
struct Doc {
    value: String,
}

impl Doc {
    fn new() -> Doc {
        Doc {
            value: String::from(""),
        }
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn append_value(&mut self, value: String) {
        self.value += &value
    }

    fn tag(&mut self, name: &str, attrs: Vec<(&str, &str)>) -> Tag<'_> {
        Tag::new(self, name, attrs)
    }
}

/// Starts a tag, which is closed automatically.
struct Tag<'a> {
    doc: &'a mut Doc,
    name: String,
}

impl<'a> Tag<'a> {
    fn new(doc: &'a mut Doc, name: &str, attrs: Vec<(&str, &str)>) -> Tag<'a> {
        doc.append_value(format!("<{}", name));
        for attr in attrs {
            let key = attr.0;
            let value = html_escape::encode_double_quoted_attribute(&attr.1);
            doc.append_value(format!(" {}=\"{}\"", key, value));
        }
        doc.append_value(String::from(">"));
        Tag {
            doc,
            name: name.to_string(),
        }
    }

    fn text(&mut self, text: &str) {
        let encoded = html_escape::encode_safe(text).to_string();
        self.doc.append_value(encoded);
    }
}

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        self.doc.append_value(format!("</{}>", self.name));
    }
}

fn main() {
    let mut doc = Doc::new();
    {
        let mut tag = doc.tag("a", vec![("href", "https://www.example.com/\"x")]);
        tag.text("here>y");
    }
    println!("value is '{}'", doc.get_value());
}
