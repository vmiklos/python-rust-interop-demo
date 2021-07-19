#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! Generate HTML with Rust.

/// Generates xml/html documents.
struct Doc {
    value: String,
}

impl Doc {
    fn new() -> Doc {
        Doc{value: String::from("")}
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn append_value(&mut self, value: String) {
        self.value += &value
    }

    fn tag(&mut self, name: String, attrs: Vec<(String, String)>) -> Tag<'_> {
        Tag::new(self, name, attrs)
    }
}

/// Starts a tag, which is closed automatically.
struct Tag<'a> {
    doc: &'a mut Doc,
    name: String,
}

impl<'a> Tag<'a> {
    fn new(doc: &mut Doc, name: String, attrs: Vec<(String, String)>) -> Tag<'_> {
        doc.append_value(String::from("<") + &name);
        for attr in attrs {
            doc.append_value(format!(" {}=\"{}\"", attr.0, attr.1));
        }
        doc.append_value(String::from(">"));
        Tag{doc, name}
    }

    fn text(&mut self, text: String) {
        self.doc.append_value(text);
    }
}

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        self.doc.append_value(String::from("</") + &self.name + &String::from(">"));
    }
}

fn main() {
    let mut doc = Doc::new();
    {
        let mut tag = doc.tag(String::from("a"), vec![(String::from("href"), String::from("https://www.example.com/"))]);
        tag.text(String::from("here"));
    }
    println!("value is '{}'", doc.get_value());
}
