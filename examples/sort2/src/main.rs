use rust_icu_ustring as ustring;
use rust_icu_ucol as ucol;
use std::convert::TryFrom;

fn get_sort_key(bytes: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let collator = ucol::UCollator::try_from("hu")?;
    let string = ustring::UChar::try_from(&(*bytes)[..])?;
    Ok(collator.get_sort_key(&string))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut strings = vec![String::from("Kőpor"), String::from("Kórház")];

    strings.sort_by(|a, b| {
        let mut a_key = a.as_bytes().to_vec();
        if let Ok(value) = get_sort_key(&a) {
            a_key = value;
        }
        let mut b_key = b.as_bytes().to_vec();
        if let Ok(value) = get_sort_key(&b) {
            b_key = value;
        }
        a_key.cmp(&b_key)
    });

    println!("strings is {:?}", strings);
    Ok(())
}
