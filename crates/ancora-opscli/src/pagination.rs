use crate::run_store::RunEntry;

pub struct Page<'a> {
    pub items: Vec<&'a RunEntry>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
}

impl<'a> Page<'a> {
    pub fn has_next(&self) -> bool {
        (self.page + 1) * self.page_size < self.total
    }
}

pub fn paginate<'a>(entries: &'a [RunEntry], page: usize, page_size: usize) -> Page<'a> {
    let total = entries.len();
    let start = page * page_size;
    let items = entries
        .iter()
        .skip(start)
        .take(page_size)
        .collect::<Vec<_>>();
    Page { items, total, page, page_size }
}
