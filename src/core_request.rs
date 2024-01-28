use crate::constants::BASE_URL;
use ureq;
use url;
/// Handles all outgoing requests to JobStreet.
pub fn make_request(url: url::Url) -> Option<String> {
    let agent = ureq::AgentBuilder::new().build();
    let resp = agent.get(url.as_str()).call().unwrap();
    match resp.into_string() {
        Err(e) => {
            println!("{e}");
            None
        }
        Ok(page) => Some(page),
    }
}
/// Parses url into valid JobStreet links.
/// - `to_base` is set to true if the `sanitized` string is already a valid path to a JobStreet page.
pub fn parse_url(sanitized: String, to_base: bool) -> url::Url {
    // https://www.jobstreet.co.id/id/python-jobs?sortmode=ListedDate
    let url = url::Url::parse(BASE_URL).unwrap();
    if to_base {
        let to_return = url.join(sanitized.as_str()).unwrap();
        return to_return;
    }
    let add_region = url.join("/id").unwrap();
    add_region.join(sanitized.as_str()).unwrap()
}
