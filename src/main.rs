use std::io::IntoInnerError;

use ureq;
use select::{document::Document, predicate::*};
use url;
fn main() {
    let to_search = "python";
    let sanitized = sanitize_input(to_search);
    let url = parse_url(sanitized);
    match make_request(url) {
        None => println!("Page not Found"),
        Some(page) => new_scrape(page),
    }
    
}
fn sanitize_input(keyword: &str) -> String {
    let split = keyword
        .trim()
        .split(" ")
        .collect::<Vec<&str>>();
    let base_string = split.join("-");
    format!("{base_string}-jobs")
    
}
fn parse_url(sanitized: String) -> url::Url {
    // https://www.jobstreet.co.id/id/python-jobs?sortmode=ListedDate    
    let url =  url::Url::parse("https://www.jobstreet.co.id/id/").unwrap();
    url.join(sanitized.as_str()).unwrap()
    
}
fn make_request(url: url::Url) -> Option<String> {
    let agent = ureq::AgentBuilder::new().build();
    let resp = agent.get(url.as_str()).call().unwrap();
    match resp.into_string() {
        Err(e) => {
            println!("{e}");
            None
            },
        Ok(page) => Some(page)
    } 
}
fn new_scrape(page :String) {
    let doc = Document::from(page.as_str());
    let mut divs = Vec::new();
    for div in doc.find(Name("div")){
        divs.push(div);
    }
    let mut main_div = Vec::new();
    for div in divs {
        match div.attr("role") {
            None => (),
            Some(attr) => {
                if attr == "main" {
                    main_div.push(div)
                }
            }
        }
    }
    let div_role_main_doc = Document::from(main_div[0].html().as_str());
    
    let mut parent_section = Vec::new();
    for section in div_role_main_doc.find(Name("section")) {
        // sticky-save-search-desktop
        match section.attr("name") {
            None => (),
            Some(attr) => {
                if attr == "sticky-save-search-desktop" {
                    parent_section.push(section)
                }
            }
        }
    }
    let mut div_in_section = Vec::new();
    let _ = parent_section[0]
        .children()
        .map(|children| {
            // 2 childs
            div_in_section.push(children);
        })
        .collect::<Vec<()>>();
    let mut inner_div = Vec::new();
    for div in div_in_section[1].find(Name("div")) {
        match div.attr("class") {
            None => (),
            Some(attr) => {
                if attr == "_1wkzzau0 a1msqi5e a1msqi5a a1msqiga a1msqi8i a1msqi8j a1msqi8c"{
                    for div2 in div.children() {
                        inner_div.push(div2)
                    }
                }
            }
        }
    }
    
    
}