pub struct CreateLanReq {
    pub name: String,
    pub prefix: String
}

pub struct RenameLanReq {
    pub id: String,
    pub name: String
}

pub struct RenameLanByNameReq {
    pub name: String,
    pub new_name: String 
}

pub struct SetLanPrefixReq {
    pub id: String,
    pub prefix: String
}

pub struct DeleteLanReq {
    pub id: String,
    // pub used_by_snat: HashSet<SNatName>
}
