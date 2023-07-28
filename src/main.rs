// tokio main function

use tokio::{self, fs::File};
use std::io::{stdin, Read};
use tar::{Archive, Entry};
use std::path::Path;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut a = Archive::new(stdin());
    //let mut futures = Vec::new();
    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        let path = Path::new("test").join(file.header().path().unwrap());
        let size = file.header().size().unwrap();

        //check if path is directory
        if file.header().entry_type().is_dir() {
            std::fs::create_dir_all(path).unwrap();
            continue;
        }

        let mut filecontents = Vec::with_capacity(size as usize);
        file.read_to_end(&mut filecontents).unwrap();

        tokio::spawn(async move {
            // create parent
            let parent = path.parent().unwrap();
            tokio::fs::create_dir_all(parent).await.unwrap();
            let mut file = File::create(path).await.unwrap();
            file.write_all(&filecontents).await.unwrap();
        });
    }

    //futures::future::join_all(futures).await;
}
