use libc::c_int;
use libc::setlocale;
use libc::strxfrm;
use libc::LC_ALL;

fn safe_setlocale(category: c_int, locale: String) -> Result<(), Box<dyn std::error::Error>> {
    let c_str = std::ffi::CString::new(locale.as_bytes())?;
    unsafe {
        setlocale(category, c_str.as_ptr() as *const i8);
    }
    Ok(())
}

fn safe_strxfrm(src: &String) -> Result<String, Box<dyn std::error::Error>> {
    let src_cstring = std::ffi::CString::new(src.as_bytes())?;
    // Assume no change in size, first.
    let n1 = src.len() + 1;
    let mut dest: Vec<i8> = vec![0; n1];
    let mut n2 = unsafe { strxfrm(dest.as_mut_ptr(), src_cstring.as_ptr(), n1) };

    if n2 >= n1 {
        // More space needed.
        dest = vec![0; n2 + 1];
    }

    n2 = unsafe { strxfrm(dest.as_mut_ptr(), src_cstring.as_ptr(), n2 + 1) };

    Ok(String::from_utf8(
        dest[..n2].iter().map(|&c| c as u8).collect(),
    )?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut strings = vec!["Kőpor", "Kórház"];

    safe_setlocale(LC_ALL, "hu_HU.UTF-8".to_string())?;

    strings.sort_by(|a, b| {
        safe_strxfrm(&a.to_string())
            .unwrap()
            .cmp(&safe_strxfrm(&b.to_string()).unwrap())
    });

    println!("strings is '{:?}'", strings);

    Ok(())
}
