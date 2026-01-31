use serde::{Deserialize, Serialize};

/// Pagination query parameters
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,
    
    /// Items per page
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    10
}

impl PaginationParams {
    /// Create pagination params
    pub fn new(page: u32, limit: u32) -> Self {
        let page = if page < 1 { 1 } else { page };
        let limit = if limit < 1 {
            10
        } else if limit > 100 {
            100 // Max 100 items per page
        } else {
            limit
        };

        PaginationParams { page, limit }
    }

    /// Get skip count for database query
    pub fn skip(&self) -> u64 {
        ((self.page - 1) * self.limit) as u64
    }

    /// Get limit for database query
    pub fn limit(&self) -> u32 {
        self.limit
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        PaginationParams {
            page: 1,
            limit: 10,
        }
    }
}

/// Paginated response metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationMeta {
    /// Current page
    pub current_page: u32,
    
    /// Items per page
    pub per_page: u32,
    
    /// Total number of items
    pub total: u64,
    
    /// Total number of pages
    pub total_pages: u64,
    
    /// Has next page
    pub has_next: bool,
    
    /// Has previous page
    pub has_prev: bool,
}

impl PaginationMeta {
    /// Create pagination metadata
    pub fn new(page: u32, limit: u32, total: u64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as u64;
        let has_next = (page as u64) < total_pages;
        let has_prev = page > 1;

        PaginationMeta {
            current_page: page,
            per_page: limit,
            total,
            total_pages,
            has_next,
            has_prev,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params_defaults() {
        let params = PaginationParams::default();
        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 10);
    }

    #[test]
    fn test_pagination_params_skip() {
        let params = PaginationParams::new(2, 10);
        assert_eq!(params.skip(), 10);

        let params = PaginationParams::new(3, 20);
        assert_eq!(params.skip(), 40);
    }

    #[test]
    fn test_pagination_params_limits() {
        let params = PaginationParams::new(0, 10);
        assert_eq!(params.page, 1); // Should be at least 1

        let params = PaginationParams::new(1, 200);
        assert_eq!(params.limit, 100); // Should be capped at 100
    }

    #[test]
    fn test_pagination_meta() {
        let meta = PaginationMeta::new(1, 10, 25);
        assert_eq!(meta.total_pages, 3);
        assert!(meta.has_next);
        assert!(!meta.has_prev);

        let meta = PaginationMeta::new(3, 10, 25);
        assert!(!meta.has_next);
        assert!(meta.has_prev);
    }
}
