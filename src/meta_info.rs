pub struct MetaInfo {
    pub announce: String,
    pub announce_list: Vec<Vec<String>>,
    pub creation_date: u32,
    pub comment: String,
    pub created_by: String,
    pub encoding: String,
    pub pieces_length: u32,
    pub pieces: String,
    pub private: bool,
    pub info: InfoType,
}

pub enum InfoType {
    single(Single),
    multiple(Multiple),

    unset,
}

struct Single {
    name: String,
    length: u32,
    md5: String
}

struct Multiple {
    name: String,
    files: Vec<Files>
}

struct Files {
    length: u32,
    md5: String,
    path: Vec<String>
}