use curl::easy::Easy;

/// EXAMPLE
/// let url: String = String::from("https://qlsv4412xi.execute-api.us-east-1.amazonaws.com/default/ClientIP");
/// let resp_str: String = get_url(&url);
/// println!("url={}, response={}", &url, &resp_str);
pub fn get_url(url: &String) -> String {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    {
    easy.url(&url).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    }
    let resp_str = String::from_utf8(dst).unwrap();
    // return the response
    resp_str
}