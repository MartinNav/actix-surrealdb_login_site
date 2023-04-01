///Wrapper over `std::fs::read()`, you have to handle the errors by your self
pub fn read_file(dir: String) -> Result<String, std::io::Error> {
    let content = String::from_utf8_lossy(&std::fs::read(dir)?).to_string();
    Ok(content)
}
/// # safe_read_file
///This is safe implementation of `read_file()` function
///You can not change the error mesage
///## Proper usecase:
/// ```
///let site_content:String = read_file("Directory/page.html".to_string());
/// ```
pub fn safe_read_file(dir: String) -> String {
    let f = read_file(dir);
    match f {
        Ok(d) => return d,
        _ => return format!("<h1>Oops</h1><p>Something went terribly wrong.....</p>"),
    }
}

///# read_with_custom_error_message
///## Use case
/// ```
///let site_content:String = read_with_custom_error_message(
///                             "Your/page.html".to_string(),
///                             "404 Not found".to_string(),
///                             "Not found".to_string());
/// ```
pub fn read_with_custom_error_message(dir: String, err_code: String, err_msg: String) -> String {
    let f = read_file(dir);
    match f {
        Ok(d) => d,
        Err(_e) => format!("<h1>{}</h1><p>{}</p>", err_code, err_msg),
    }
}
