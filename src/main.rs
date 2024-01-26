use jobstreet_jobs::{*, job_details::details_main, core_request::{make_request, parse_url}};
use select::{document::Document, predicate::*};

fn main() {
    let to_search = "javascript";
    let sanitized = sanitize_input(to_search);
    let url = parse_url(sanitized, false);
    match make_request(url) {
        None => println!("Page not Found"),
        Some(page) => {
            let job_cards = get_job_cards(page);
            let list_of_jobs = get_preliminary_info(job_cards);
            details_main(list_of_jobs)
        }
    }
}
fn sanitize_input(keyword: &str) -> String {
    let split = keyword.trim().split(" ").collect::<Vec<&str>>();
    let base_string = split.join("-");
    format!("{base_string}-jobs")
}
fn get_preliminary_info(vector: Vec<String>) -> Vec<jobstreet_jobs::JobPage> {
    let mut articles = Vec::new();
    for page in vector.iter() {
        let doc = Document::from(page.as_str());
        let find_article = doc.find(Name("article"));
        for article in find_article {
            if let Some("JobCard") = article.attr("data-card-type") {
                let job_name = match article.attr("aria-label") {
                    None => {
                        println!("Job title not found");
                        "None"
                    }
                    Some(job_name) => job_name,
                };
                articles.push(JobCard::new(job_name.to_string(), article.html()));
            }
        }
    }
    let mut job_title_and_link = Vec::new();
    for job in articles.iter() {
        let page_doc = Document::from(job.get_page().as_str());
        let mut date: String = format!("None");
        let article_last_child = page_doc
            .find(Name("article"))
            .next()
            .and_then(|article| {
                if let Some("JobCard") = article.attr("data-card-type") {
                    date = match article.find(Attr("class", "_1wkzzau0 a1msqi5i a1msqi0 _6ly8y50")).next() {
                        None => "None".to_string(),
                        Some(node) => {
                            node.text()
                        }
                    };
                    article.last_child()
                } else {
                    None
                }
            });
        if let Some(last_child) = article_last_child {
            for div in last_child.find(Name("div")) {
                if let Some("_1wkzzau0 szurmz0 szurmz4") = div.attr("class") {
                    let link = div
                        .last_child()
                        .and_then(|node|{
                            node.find(Attr("data-automation", "jobTitle")).next()   
                        })
                        .and_then(|a| {
                            a.attr("href")
                        })
                        .and_then(|attr| {
                            Some(attr)
                        });
                    if let Some(attr) = link {
                        job_title_and_link.push(JobPage::new(job.get_title(), date.clone() ,attr.to_string()));
                    }
                } 
            }
        }
    }
    job_title_and_link
}

fn get_job_cards(page: String) -> Vec<String> {
    let doc = Document::from(page.as_str());
    let mut divs = Vec::new();
    for div in doc.find(Name("div")) {
        divs.push(div);
    }
    let mut main_div = Vec::new();
    for div in divs {
        if let Some("main") = div.attr("role") {
            main_div.push(div)
        }
    }
    let div_role_main_doc = Document::from(main_div[0].html().as_str());

    let mut parent_section = Vec::new();
    for section in div_role_main_doc.find(Name("section")) {
        if let Some("sticky-save-search-desktop") = section.attr("name") {
            parent_section.push(section)
        }
    }
    let mut div_in_section = Vec::new();
    for child in parent_section[0].children() {
        div_in_section.push(child);
    }
    let mut inner_div = Vec::new();
    for div in div_in_section[1].find(Name("div")) {
        if let Some("_1wkzzau0 a1msqi5e a1msqi5a a1msqiga a1msqi8i a1msqi8j a1msqi8c") =
            div.attr("class")
        {
            div.children()
                .filter(|child| {
                    child.is(Name("div"))
                        && child.attr("class").unwrap_or("None") == "_1wkzzau0 _21bfxf1"
                })
                .map(|child| inner_div.push(child))
                .collect()
        }
    }
    let final_node = inner_div
        .first()
        .and_then(|node| node.children().next())
        .and_then(|node| node.last_child());

    let mut to_return = Vec::new();
    let mut lwk_children = Vec::new();
    match final_node {
        None => (),
        Some(node) => lwk_children = node.children().collect(),
    }
    for child in lwk_children[1].children() {
        to_return.push(child.html());
    }
    to_return
}