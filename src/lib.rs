extern crate libc;
use libc::c_char;
use serde::Deserialize;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::path::Path;

/// Unsplash random image API URL
const API_URL: &str = "https://api.unsplash.com/photos/random";

/// Response from Unsplash
#[derive(Deserialize, Debug)]
struct ReturnDictionary {
    id: String,
    urls: HashMap<String, String>,
}

#[derive(Debug)]
struct UnsplashParameters {
    collections: String,
    topics: String,
    username: String,
    query: String,
    orientation: String,
    content_filter: String,
}

impl UnsplashParameters {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            collections: String::new(),
            topics: String::new(),
            username: String::new(),
            query: String::new(),
            orientation: String::new(),
            content_filter: String::new(),
        }
    }

    fn get_query(&self) -> Vec<(&str, String)> {
        let mut query = Vec::new();
        if !self.collections.is_empty() {
            query.push(("collections", self.collections.clone()));
        }
        if !self.topics.is_empty() {
            query.push(("topics", self.topics.clone()));
        }
        if !self.query.is_empty() {
            query.push(("query", self.query.clone()));
        }
        if !self.username.is_empty() {
            query.push(("username", self.username.clone()));
        }
        if !self.orientation.is_empty() {
            query.push(("orientation", self.orientation.clone()));
        } else {
            query.push(("orientation", "landscape".to_string()));
        }
        if !self.content_filter.is_empty() {
            query.push(("content_filter", self.content_filter.clone()));
        }

        return query;
    }
}

/// Save image from `url` to `file_path`
fn save_image(url: &String, file_path: &String) {
    let mut file = std::fs::File::create(file_path).unwrap();
    reqwest::blocking::get(url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
}

/// Make request to Unsplash API
fn request_new_image(
    api_key: &str,
    query: &Vec<(&str, String)>,
) -> Result<ReturnDictionary, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp: ReturnDictionary = client
        .get(API_URL)
        .query(query)
        .header("Authorization", format!("Client-ID {}", api_key))
        .send()?
        .json()?;

    return Ok(resp);
}

#[no_mangle]
/// Save new image to `raw_folder` using `raw_api_key`
pub fn save_new_image(
    raw_api_key: *const c_char,
    raw_folder: *const c_char,
    collections: *const c_char,
    topics: *const c_char,
    username: *const c_char,
    query: *const c_char,
    orientation: *const c_char,
    content_filter: *const c_char,
) -> *const c_char {
    // Convert C string to Rust string
    let api_key_c = unsafe { CStr::from_ptr(raw_api_key) };
    let api_key = api_key_c.to_str().unwrap();
    let folder_c = unsafe { CStr::from_ptr(raw_folder) };
    let folder = folder_c.to_str().unwrap().to_string();

    let collections_c = unsafe { CStr::from_ptr(collections) };
    let topics_c = unsafe { CStr::from_ptr(topics) };
    let username_c = unsafe { CStr::from_ptr(username) };
    let query_c = unsafe { CStr::from_ptr(query) };
    let orientation_c = unsafe { CStr::from_ptr(orientation) };
    let content_filter_c = unsafe { CStr::from_ptr(content_filter) };

    let unsplash_params: UnsplashParameters = UnsplashParameters {
        collections: collections_c.to_str().unwrap().to_string(),
        topics: topics_c.to_str().unwrap().to_string(),
        username: username_c.to_str().unwrap().to_string(),
        query: query_c.to_str().unwrap().to_string(),
        orientation: orientation_c.to_str().unwrap().to_string(),
        content_filter: content_filter_c.to_str().unwrap().to_string(),
    };

    // Make request to Unsplash and save image
    let id: String;
    let urls: HashMap<String, String>;
    if let Ok(resp) = request_new_image(&api_key, &unsplash_params.get_query()) {
        id = resp.id;
        urls = resp.urls;
    }
    else {
        // There was a problem with the request
        return CString::new("ERROR").unwrap().into_raw();
    }
    let path: &Path = Path::new(&folder);
    let file_path = path
        .join(format!("{}.jpg", id))
        .to_str()
        .unwrap()
        .to_string();
    save_image(&urls.get("raw").unwrap().to_string(), &file_path);

    return CString::new(id).unwrap().into_raw();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn make_request_to_unsplash() {
        // For development, set UNSPLASH_API_KEY env variable
        let api_key: String = env::var("UNSPLASH_API_KEY").unwrap();
        let unsplash_params = UnsplashParameters::new();
        let image_response = request_new_image(&api_key, &unsplash_params.get_query());
        // Either the request was successful or it was not and there is a `reqwest::Error`
        assert!(image_response.is_ok() || image_response.is_err_and(|e| e.is::<reqwest::Error>()));
    }
    #[test]
    fn test_save_image() {
        let image_url = String::from("https://images.unsplash.com/photo-1461988320302-91bde64fc8e4?ixid=2yJhcHBfaWQiOjEyMDd9");
        let temp_dir = env::temp_dir().to_str().unwrap().to_string();
        let file_path = format!("{}/test.jpg", temp_dir);
        save_image(&image_url, &file_path);
    }
}
