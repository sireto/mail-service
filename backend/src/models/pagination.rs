use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Page size used when a caller does not ask for one.
pub const DEFAULT_PAGE_SIZE: i64 = 50;

/// Hard ceiling on page size.
///
/// The list endpoints used to load whole tables. `mails.mail_message` holds the full rendered
/// HTML of every message, so `GET /api/mails` grew into hundreds of megabytes after a few
/// campaigns and `contacts` pulled every list membership into memory alongside it. A cap the
/// caller cannot raise is the part that actually fixes that: an optional `limit` alone would
/// leave the unbounded response one omitted parameter away.
pub const MAX_PAGE_SIZE: i64 = 200;

/// `limit` / `offset` query parameters, with the clamping applied in one place.
#[derive(Debug, Default, Deserialize, IntoParams)]
pub struct PageQuery {
    /// Rows per page. Defaults to 50, clamped to 200.
    pub limit: Option<i64>,
    /// Rows to skip. Negative values are treated as 0.
    pub offset: Option<i64>,
}

impl PageQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, MAX_PAGE_SIZE)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

/// One page of results plus enough context for a caller to know what it is missing.
///
/// `total` is the reason this is an envelope rather than a bare array: truncating a response
/// without saying so leaves the caller unable to tell "no more rows" from "we stopped
/// telling you", which is worse than a slow endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// Total rows matching the query, ignoring limit and offset.
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    /// True when rows remain after this page.
    pub has_more: bool,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: i64, limit: i64, offset: i64) -> Self {
        let has_more = offset.saturating_add(items.len() as i64) < total;
        Self {
            items,
            total,
            limit,
            offset,
            has_more,
        }
    }

    /// Build a page from a source page while transforming each item.
    pub fn map<U, F: FnMut(T) -> U>(self, f: F) -> Page<U> {
        Page {
            items: self.items.into_iter().map(f).collect(),
            total: self.total,
            limit: self.limit,
            offset: self.offset,
            has_more: self.has_more,
        }
    }

    /// Build a page from items already transformed elsewhere, keeping this page's window.
    pub fn with_items<U>(&self, items: Vec<U>) -> Page<U> {
        Page {
            items,
            total: self.total,
            limit: self.limit,
            offset: self.offset,
            has_more: self.has_more,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_clamping() {
        let empty = PageQuery::default();
        assert_eq!(empty.limit(), DEFAULT_PAGE_SIZE);
        assert_eq!(empty.offset(), 0);

        // A caller cannot ask for the whole table any more.
        let greedy = PageQuery {
            limit: Some(100_000),
            offset: None,
        };
        assert_eq!(greedy.limit(), MAX_PAGE_SIZE);

        // Nor for a nonsensical window.
        let nonsense = PageQuery {
            limit: Some(0),
            offset: Some(-10),
        };
        assert_eq!(nonsense.limit(), 1);
        assert_eq!(nonsense.offset(), 0);

        let sane = PageQuery {
            limit: Some(25),
            offset: Some(50),
        };
        assert_eq!(sane.limit(), 25);
        assert_eq!(sane.offset(), 50);
    }

    #[test]
    fn has_more_reflects_the_window() {
        // A full page with rows behind it.
        let first = Page::new(vec![1, 2, 3], 10, 3, 0);
        assert!(first.has_more);

        // The last page.
        let last = Page::new(vec![10], 10, 3, 9);
        assert!(!last.has_more);

        // Exactly one page of results.
        let only = Page::new(vec![1, 2], 2, 50, 0);
        assert!(!only.has_more);

        // Empty result set.
        let none: Page<i32> = Page::new(vec![], 0, 50, 0);
        assert!(!none.has_more);
        assert_eq!(none.total, 0);
    }

    #[test]
    fn map_preserves_the_window() {
        let page = Page::new(vec![1, 2], 7, 2, 4);
        let mapped = page.map(|n| n.to_string());

        assert_eq!(mapped.items, vec!["1".to_string(), "2".to_string()]);
        assert_eq!(mapped.total, 7);
        assert_eq!(mapped.limit, 2);
        assert_eq!(mapped.offset, 4);
        assert!(mapped.has_more);
    }
}
