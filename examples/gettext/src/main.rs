#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! The i18n module allows UI translation via gettext.

thread_local! {
    static TRANSLATIONS: std::cell::RefCell<Option<gettext::Catalog>> = std::cell::RefCell::new(None);
}

fn tr(english: &str) -> String {
    TRANSLATIONS.with(|translations| {
        let translations = translations.borrow();
        match *translations {
            Some(ref translations) => translations.gettext(english).to_string(),
            None => english.to_string(),
        }
    })
}

fn set_language(language: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(format!("locale/{}/LC_MESSAGES/myapp.mo", language))?;
    let catalog = gettext::Catalog::parse(file)?;
    TRANSLATIONS.with(|translations| {
        *translations.borrow_mut() = Some(catalog);
    });
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = set_language("hu");
    println!("{}", tr("Name"));
    Ok(())
}
