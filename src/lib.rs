pub struct JobCard {
    title: String,
    page: String,
}
impl JobCard {
    pub fn new(t: String, p: String) -> JobCard {
        JobCard { title: t, page: p }
    }
    pub fn get_title(self) -> String {
        self.title
    }
    pub fn get_page(self) -> String {
        self.page
    }
}
