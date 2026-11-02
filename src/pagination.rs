//! Pages of a list endpoint. The API pages by `page` and `page_size`, one-indexed, and every list response
//! carries the page it returned under `data.pagination`.

use crate::models::PaginationMeta;

/// One page of a list, with the items it carries and what the API says about the pages around it.
#[derive(Debug, Clone, PartialEq)]
pub struct Page<T> {
    /// The items of this page.
    pub items: Vec<T>,
    /// The page number, size, totals, and whether a page follows.
    pub pagination: PaginationMeta,
}

impl<T> Page<T> {
    /// Whether a page follows this one.
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.pagination.has_more.unwrap_or(false)
    }
}

/// The query parameters of a paginated list: the page they ask for.
pub trait PageParams {
    /// The page asked for, one-indexed; None is the first.
    fn page(&self) -> Option<i32>;
    /// Ask for a page.
    fn set_page(&mut self, page: i32);
}
