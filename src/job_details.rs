use crate::JobPage;
pub fn details_main(url: Vec<JobPage>) {
    for job in url.iter() {
        println!("{job:?}");
    }
}