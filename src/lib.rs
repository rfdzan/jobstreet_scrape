#[derive(Debug, Clone)]
pub struct JobCard {
    title: String,
    page: String,
}
impl JobCard {
    pub fn new(t: String, p: String) -> JobCard {
        JobCard { title: t, page: p }
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_page(&self) -> String {
        self.page.clone()
    }
}
#[derive(Debug, Clone)]
pub struct JobPage {
    title: String,
    link: String,
}
impl JobPage {
    pub fn new(t: String, l: String) -> JobPage {
        JobPage {
            title: t,
            link: l
        }
    }
}
