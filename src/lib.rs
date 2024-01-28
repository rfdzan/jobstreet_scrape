//! JobStreet Job Scrape
//! 

pub mod job_details;
pub mod core_request;
pub mod constants;
/// Contains job title information from JobStreet search page, along with html of each cards.
#[derive(Debug, Clone)]
pub struct JobCard {
    title: String,
    page: String,
}
impl JobCard {
    /// Creates new JobCard.
    pub fn new(t: String, p: String) -> JobCard {
        JobCard { title: t, page: p }
    }
    /// Returns the job title.
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    /// Returns the job card html.
    pub fn get_page(&self) -> String {
        self.page.clone()
    }
}
/// More detailed information from job cards, with links to each job posting page.
#[derive(Debug, Clone)]
pub struct JobPage {
    title: String,
    date: String,
    link: String,
}
impl JobPage {
    /// Creates new JobPage.
    pub fn new(t: String, d: String, l: String) -> JobPage {
        JobPage {
            title: t,
            date: d,
            link: l,
        }
    }
}
