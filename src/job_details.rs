use crate::{JobPage, core_request::*};
use select::{document::Document, predicate::*};

pub fn details_main(url: Vec<JobPage>) {
    // if let Some(page) = make_request(url) {
    //         scrape_details(page);
    // }
    for job in url.iter() {
        let url = parse_url(job.link.clone(), true);
        if let Some(page) = make_request(url) {
            scrape_details(page);
        }
    }
}
fn scrape_details(page: String) {
    let doc = Document::from(page.as_str());
    let all_divs = doc.find(Name("div"));
    let mut main_div = String::new();
    for div in all_divs {
        if let Some("main") = div.attr("role") {
            main_div = div.html();
        }
    }
    let main_div_doc = Document::from(main_div.as_str());
    let mut data_sticky_child = String::new();
    for div in  main_div_doc.find(Name("div")) {
        if let Some("job-details-page") = div.attr("data-sticky") {
            if let Some(child) = div.first_child() {
                data_sticky_child = child.html();
            }
        }
    }
    let szurmz7_doc = Document::from(data_sticky_child.as_str());
    let mut div1 = String::new();
    for div in szurmz7_doc.find(Name("div")) {
        if let Some("_1wkzzau0 szurmz0 szurmz9") = div.attr("class") {
            if let Some(div_first_child ) = div.first_child() {
                if let Some("_1wkzzau0 a1msqi76") = div_first_child.attr("class") {
                    div1 = div_first_child.html()
                }
            }
        }
    }
    let div1_doc = Document::from(div1.as_str());
    let mut third_div_first_child = Vec::with_capacity(1);
    for div in div1_doc.find(Name("div")) {
        if let Some("_1wkzzau0 szurmz0 szurmz7") = div.attr("class") {
            if let Some(third_div) = div.children().take(3).last() {
                if let Some(first_child_of_third_div) = third_div.first_child() {
                    third_div_first_child.push(first_child_of_third_div);
                }
            }
        }
    }
    let mut should_be_4 = Vec::with_capacity(4);
    if let Some(first_child_of_third_div) = third_div_first_child.iter().next() {
        if let Some("_1wkzzau0 szurmz0 szurmz6") = first_child_of_third_div.attr("class") {
            for child in first_child_of_third_div.children() {
                should_be_4.push(child);
            }
        }
    }
    for node in should_be_4.iter() {
        for span in node.find(Name("span")) {
            println!("{}", span.text());
        }
    }
}