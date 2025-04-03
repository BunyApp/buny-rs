use crate::std::{ObjectRef, Rid, String, ValueRef, Vec};

#[link(wasm_import_module = "buny")]
unsafe extern "C" {
    fn create_novel(
        id: *const u8,
        id_len: usize,
        cover_url: *const u8,
        cover_url_len: usize,
        title: *const u8,
        title_len: usize,
        author: *const u8,
        author_len: usize,
        artist: *const u8,
        artist_len: usize,
        description: *const u8,
        description_len: usize,
        url: *const u8,
        url_len: usize,
        categories: *const *const u8,
        category_str_lens: *const usize,
        category_count: usize,
        status: NovelStatus,
        nsfw: NovelContentRating,
        viewer: NovelViewer,
    ) -> i32;

    unsafe fn create_novel_result(novel_array: Rid, has_more: bool) -> i32;

    unsafe fn create_novel_reviews(
        username: *const u8,
        username_len: usize,
        cover: *const u8,
        cover_len: usize,
        content: *const u8,
        content_len: usize,
        rating: f32,
        date_str: *const u8,
        date_str_len: usize,
    ) -> i32;

    fn create_chapter(
        id: *const u8,
        id_len: usize,
        title: *const u8,
        title_len: usize,
        volume: f32,
        chapter: f32,
        date_updated: f64,
        scanlator: *const u8,
        scanlator_len: usize,
        url: *const u8,
        url_len: usize,
        lang: *const u8,
        lang_len: usize,
    ) -> i32;

    fn create_chapter_result(chapter_array: Rid, has_more: bool) -> i32;

    fn create_chapter_content(index: i32, paragraph: *const u8, paragraph_len: usize) -> i32;
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum FilterType {
    #[default]
    Base = 0,
    Group = 1,
    Text = 2,
    Check = 3,
    Select = 4,
    Sort = 5,
    SortSelection = 6,
    Title = 7,
    Author = 8,
    Genre = 9,
}

impl From<i32> for FilterType {
    fn from(value: i32) -> Self {
        Self::from(value as i64)
    }
}

impl From<i64> for FilterType {
    fn from(value: i64) -> Self {
        match value {
            0 => FilterType::Base,
            1 => FilterType::Group,
            2 => FilterType::Text,
            3 => FilterType::Check,
            4 => FilterType::Select,
            5 => FilterType::Sort,
            6 => FilterType::SortSelection,
            7 => FilterType::Title,
            8 => FilterType::Author,
            9 => FilterType::Genre,
            _ => FilterType::Base,
        }
    }
}

impl FilterType {
    pub fn to_int(&self) -> i32 {
        match self {
            FilterType::Base => 0,
            FilterType::Group => 1,
            FilterType::Text => 2,
            FilterType::Check => 3,
            FilterType::Select => 4,
            FilterType::Sort => 5,
            FilterType::SortSelection => 6,
            FilterType::Title => 7,
            FilterType::Author => 8,
            FilterType::Genre => 9,
        }
    }
}

/// An enum representing the various statuses a novel can have.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum NovelStatus {
    /// The novel status cannot be determined.
    #[default]
    Unknown = 0,

    /// A novel that is still releasing chapters/being translated.
    Ongoing = 1,

    /// A novel that has completed production/translation.
    Completed = 2,

    /// A novel that has been cancelled. This could convey the novel
    /// being dropped, or the translation team has stopped working on the novel,
    /// even though the novel itself is still ongoing.
    Cancelled = 3,

    /// The novel is on hiatus. Could happen because the author decided
    /// to get a PS5 and then leave people on a cliffhanger for two years
    /// straight.
    Hiatus = 4,
}

/// An enumeration representing the novel's content rating.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum NovelContentRating {
    #[default]
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

/// An enumeration representing different novel viewers, used to indicate
/// the preferred reading method for this novel.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum NovelViewer {
    #[deprecated(
        since = "0.2.0",
        note = "novelViewer::Default is ignored in the app, and defaults to the RTL viewer."
    )]
    Default = 0,
    #[default]
    Rtl = 1,
    Ltr = 2,
    Vertical = 3,
    Scroll = 4,
}

/// Struct representing a search filter.
#[derive(Clone, Default)]
pub struct Filter {
    /// The filter variant.
    pub kind: FilterType,

    /// The filter's name, which matches the name of the filter in `filters.json`.
    pub name: String,

    /// The filter's value. This is dependent on what the filter type is.
    pub value: ValueRef,

    /// The raw filter object.
    pub object: ObjectRef,
}

/// The novel struct contains information about a novel. Different novels
/// are differentiated by their ID, and so changing the ID will result in
/// a different novel. Thus, developers should decide on the ID format
/// before publishing their source.
///
/// The ID must be unique at the source-level.
#[derive(Clone, Debug, Default)]
pub struct Novel {
    /// The given identifier of this novel, which can be anything, from a number
    /// to the entire URL.
    pub id: String,

    /// A URL pointing to a thumbnail which can be used to display the novel.
    pub cover: String,

    /// The title of the novel. It can be either the official title, or the localized
    /// title, depending on which one the developer thinks fits best for the source.
    pub title: String,

    /// The name of the novel's author. Multiple authors should be concatenated into
    /// a single string using a delimiter such as a comma.
    pub author: String,

    /// The name of the novel's artist. Multiple artists should be concatenated into
    /// a single string using a delimiter such as a comma.
    pub artist: String,

    /// A description for this novel.
    pub description: String,

    /// The URL for this novel. Will be used for sharing and for opening the in-app
    /// browser.
    pub url: String,

    /// A vector containing all the novel's tags/categories.
    pub categories: Vec<String>,

    /// The status of the novel (completed, ongoing, hiatus, cancelled...).
    pub status: NovelStatus,

    /// The novel's content rating (safe, suggestive or NSFW).
    pub nsfw: NovelContentRating,

    /// The viewer to use for this novel.
    pub viewer: NovelViewer,
}

impl PartialEq for Novel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A struct representing a "page" of novles. There is no limit on how many novles
/// can a page have, that is up to the source to decide.
#[derive(Clone, Debug, Default)]
pub struct NovelPageResult {
    /// The novels that were found in the page.
    pub novel: Vec<Novel>,

    /// Whether there are any more pages after this one. Used to determine if
    /// the app should make another request when the user scrolls to the bottom.
    pub has_more: bool,
}

#[derive(Clone, Debug)]
pub struct Review {
    /// User's username.
    pub username: String,
    /// User's profile picture url.
    pub cover: String,
    /// The actual review for the novel.
    pub content: String,
    /// Rating from 0.0 - 1.0
    pub rating: f32,
    /// date as a string (e.g. 1/1/11, 3 days ago, 1 hour ago).
    /// The source dev has the freedom to choose the display type or leave it empty.
    pub date_string: String,
}

impl Default for Review {
    fn default() -> Self {
        Review {
            username: String::new(),
            cover: String::new(),
            content: String::new(),
            rating: 1.0,
            date_string: String::new(),
        }
    }
}

/// Struct containing information about a listing.
#[derive(Clone, Debug)]
pub struct Listing {
    /// The name of the listing.
    pub name: String,
}

impl PartialEq for Listing {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// Struct containing metadata about a chapter. Different chapters are differentiated
/// by their ID. Thus, changing the ID will result in a different chapter, even if the
/// chapters have the same volume/chapter number.
///
/// The ID must be unique at the novel level.
#[derive(Clone, Debug)]
pub struct Chapter {
    /// The given identifier of this novel, which can be anything, from a number
    /// to the entire URL.
    pub id: String,

    /// The title of the chapter.
    pub title: String,

    /// The volume that the chapter belongs to.
    pub volume: f32,

    /// The chapter number of the chapter.
    pub chapter: f32,

    /// The publishing date of the chapter.
    pub date_updated: f64,

    /// The scanlator/scanlation group that posted the chapter.
    pub scanlator: String,

    /// The chapter URL, which will be used for sharing in the future.
    pub url: String,

    /// The chapter's language. It should be a valid language code.
    pub lang: String,
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Default for Chapter {
    fn default() -> Self {
        Chapter {
            id: String::new(),
            title: String::new(),
            volume: -1.0,
            chapter: -1.0,
            date_updated: -1.0,
            scanlator: String::new(),
            url: String::new(),
            lang: String::new(),
        }
    }
}

/// Struct represents a "page" of chapters, unlike manga, novels have thousands
/// of chapters across multiple page urls and it is unfesable to load tens of
/// pages in the background while the user waits, thus chapters are loaded on request.
#[derive(Clone, Debug, Default)]
pub struct ChapterPageResult {
    /// Chapters in a page.
    pub chapters: Vec<Chapter>,
    /// does the novel have any more pages of chapters.
    pub has_more: bool,
}

/// A chapter is going to be sectioned off into paragraphs
#[derive(Clone, Debug, Default)]
pub struct ChapterParagraph {
    /// The index of the page, starting from 0.
    pub index: i32,
    /// A section or a paragraph of text on the page, where the page contains
    /// multiple paragraphs.
    pub paragraph: String,
}

impl Novel {
    pub fn create(self) -> i32 {
        let categories_ptr = &self
            .categories
            .iter()
            .map(|x| x.as_ptr())
            .collect::<Vec<*const u8>>();
        let category_lens = self
            .categories
            .iter()
            .map(|x| x.len())
            .collect::<Vec<usize>>();
        unsafe {
            create_novel(
                self.id.as_ptr(),
                self.id.len(),
                self.cover.as_ptr(),
                self.cover.len(),
                self.title.as_ptr(),
                self.title.len(),
                self.author.as_ptr(),
                self.author.len(),
                self.artist.as_ptr(),
                self.artist.len(),
                self.description.as_ptr(),
                self.description.len(),
                self.url.as_ptr(),
                self.url.len(),
                categories_ptr.as_ptr(),
                category_lens.as_ptr(),
                self.categories.len(),
                self.status,
                self.nsfw,
                self.viewer,
            )
        }
    }
}

impl NovelPageResult {
    pub fn create(self) -> i32 {
        let mut arr = buny_imports::ArrayRef::new();
        for novel in self.novel {
            let novel_descriptor = novel.create();
            arr.insert(ValueRef::new(novel_descriptor));
        }
        unsafe { create_novel_result(arr.0 .0, self.has_more) }
    }
}

impl Review {
    pub fn create(self) -> i32 {
        unsafe {
            create_novel_reviews(
                self.username.as_ptr(),
                self.username.len(),
                self.cover.as_ptr(),
                self.cover.len(),
                self.content.as_ptr(),
                self.content.len(),
                self.rating,
                self.date_string.as_ptr(),
                self.date_string.len(),
            )
        }
    }
}

impl ChapterPageResult {
    pub fn create(self) -> i32 {
        let mut arr = buny_imports::ArrayRef::new();
        for chapter in self.chapters {
            let chapter_descriptor = chapter.create();
            arr.insert(ValueRef::new(chapter_descriptor));
        }
        unsafe { create_chapter_result(arr.0 .0, self.has_more) }
    }
}

impl Chapter {
    #[inline]
    pub fn create(self) -> i32 {
        unsafe {
            create_chapter(
                self.id.as_ptr(),
                self.id.len(),
                self.title.as_ptr(),
                self.title.len(),
                self.volume,
                self.chapter,
                self.date_updated,
                self.scanlator.as_ptr(),
                self.scanlator.len(),
                self.url.as_ptr(),
                self.url.len(),
                self.lang.as_ptr(),
                self.lang.len(),
            )
        }
    }
}

impl ChapterParagraph {
    #[inline]
    pub fn create(self) -> i32 {
        unsafe { create_chapter_content(self.index, self.paragraph.as_ptr(), self.paragraph.len()) }
    }
}
