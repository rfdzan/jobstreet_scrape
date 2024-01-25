use jobstreet_jobs::*;
use select::{document::Document, node::Node, predicate::*};
use ureq;
use url;
fn main() {
    let to_search = "python";
    let sanitized = sanitize_input(to_search);
    let url = parse_url(sanitized);
    match make_request(url) {
        None => println!("Page not Found"),
        Some(page) => {
            let job_cards = get_job_cards(page);
            let _ = get_info(job_cards);
        }
    }
}
fn sanitize_input(keyword: &str) -> String {
    let split = keyword.trim().split(" ").collect::<Vec<&str>>();
    let base_string = split.join("-");
    format!("{base_string}-jobs")
}
fn parse_url(sanitized: String) -> url::Url {
    // https://www.jobstreet.co.id/id/python-jobs?sortmode=ListedDate
    let url = url::Url::parse("https://www.jobstreet.co.id/id/").unwrap();
    url.join(sanitized.as_str()).unwrap()
}
fn make_request(url: url::Url) -> Option<String> {
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
fn get_info(vector: Vec<String>) {
    let mut articles = Vec::new();
    for page in vector.into_iter() {
        let doc = Document::from(page.as_str());
        let find_article = doc.find(Name("article"));
        for article in find_article {
            match article.attr("data-card-type") {
                None => (),
                Some(attr) => {
                    if attr != "JobCard" {
                        continue;
                    }
                    let job_name = match article.attr("aria-label") {
                        None => "None",
                        Some(job_name) => job_name,
                    };
                    articles.push(JobCard::new(job_name.to_string(), article.html()));
                }
            }
        }
    }
    for job in articles.into_iter() {
        println!("{}", job.get_title())
    }
}
fn get_job_cards(page: String) -> Vec<String> {
    let doc = Document::from(page.as_str());
    let mut divs = Vec::new();
    for div in doc.find(Name("div")) {
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
                if attr == "_1wkzzau0 a1msqi5e a1msqi5a a1msqiga a1msqi8i a1msqi8j a1msqi8c" {
                    div.children()
                        .filter(|child| {
                            child.is(Name("div"))
                                && child.attr("class").unwrap_or("None") == "_1wkzzau0 _21bfxf1"
                        })
                        .map(|child| inner_div.push(child))
                        .collect()
                }
            }
        }
    }
    let mut final_node = Vec::new();
    match inner_div[0].children().next() {
        None => (),
        Some(node) => match node.last_child() {
            None => (),
            Some(last_node) => final_node.push(last_node),
        },
    }
    let mut to_return = Vec::new();
    let lwk = final_node[0].children().collect::<Vec<Node>>();
    for child in lwk[1].children() {
        to_return.push(child.html());
    }
    to_return
}
