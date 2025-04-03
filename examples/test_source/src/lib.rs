#![no_std]
use buny::{
    error::Result,
    prelude::*,
    std::{net::Request, String, Vec},
    ChapterPageResult, ChapterParagraph, Filter, Listing, Novel, NovelPageResult, Review, Chapter
};

#[initialize]
fn initialize() {
    // Place any code that is supposed to run once when the source starts here.
    // This include initializing any variables, setting the rate limit, etc.
    todo!()
}

#[get_novel_list]
fn get_novel_list(_page: i32) -> Result<NovelPageResult> {
    todo!()
}

#[get_novel_listing]
fn get_novel_listing(_listing: Listing, _page: i32) -> Result<NovelPageResult> {
    todo!()
}

#[get_search_result]
fn get_search_result(_filters: Vec<Filter>, _page: i32) -> Result<NovelPageResult> {
    todo!()
}

#[get_novel_details]
fn get_novel_details(_id: String) -> Result<Novel> {
    todo!()
}

#[get_novel_reviews]
fn get_novel_reviews(_id: String) -> Result<Vec<Review>> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_id: String, _page: i32) -> Result<ChapterPageResult> {
    todo!()
}


#[get_chapter_content]
fn get_chapter_content(_novel_id: String, _chapter_id: String) -> Result<Vec<ChapterParagraph>> {
    todo!()
}

#[modify_image_request]
fn modify_image_request(_request: Request) {
    todo!()
}

#[handle_notification]
fn handle_notification(_notification: String) {
    todo!()
}
