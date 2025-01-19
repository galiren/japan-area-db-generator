#[derive(Debug)]
pub(crate) struct Prefecture {
    pub id: u32,
    pub name: String,
    pub read: String,
}

#[derive(Debug)]
pub(crate) struct City {
    pub id: u32,
    pub prefecture_id: u32,
    pub name: String,
    pub read: String,
}
