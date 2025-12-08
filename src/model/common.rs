//! Common model types shared across the API.

/// Pagination parameters for list endpoints.
///
/// This struct provides common pagination options used across various list endpoints
/// in the Portkey API.
#[derive(Clone, Debug, Default)]
pub struct PaginationParams<'a> {
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 20.
    pub limit: Option<i32>,

    /// Sort order by the created_at timestamp of the objects.
    /// `asc` for ascending order and `desc` for descending order.
    pub order: Option<&'a str>,

    /// A cursor for use in pagination. `after` is an object ID that defines
    /// your place in the list.
    pub after: Option<&'a str>,

    /// A cursor for use in pagination. `before` is an object ID that defines
    /// your place in the list.
    pub before: Option<&'a str>,
}

impl<'a> PaginationParams<'a> {
    /// Creates a new empty pagination params.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the limit.
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the order.
    pub fn with_order(mut self, order: &'a str) -> Self {
        self.order = Some(order);
        self
    }

    /// Sets the after cursor.
    pub fn with_after(mut self, after: &'a str) -> Self {
        self.after = Some(after);
        self
    }

    /// Sets the before cursor.
    pub fn with_before(mut self, before: &'a str) -> Self {
        self.before = Some(before);
        self
    }

    /// Converts the pagination params into query parameter tuples.
    ///
    /// Returns a vector of (key, value) tuples that can be used with
    /// URL query builders.
    pub(crate) fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(order) = self.order {
            params.push(("order", order.to_string()));
        }
        if let Some(after) = self.after {
            params.push(("after", after.to_string()));
        }
        if let Some(before) = self.before {
            params.push(("before", before.to_string()));
        }

        params
    }
}
