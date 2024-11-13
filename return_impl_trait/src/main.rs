trait FetchOps {
    fn fetch(&self);
}

struct MyFetcher;

impl FetchOps for MyFetcher {
    fn fetch(&self) {
        println!("Fetching!");
    }
}

fn create_fetcher() -> impl FetchOps {
    let fetcher = MyFetcher;
    fetcher.fetch();
    fetcher
}

fn main() {
    create_fetcher();
}
