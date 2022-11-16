pub mod comments;
pub mod likes;
pub mod pagination;
pub mod pass;
pub mod thoughts;
pub mod users;

pub fn ReplaceIPFSUrl(url: String) -> String {
    url.clone()
        .split_once("/ipfs/")
        .map_or(url, |(_, y)| "https://hug.land/ipfs/".to_string() + y)
}