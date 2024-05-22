pub struct PostHomeSDto{
    pub name: String,
    pub address: String,
    pub latitude: i64,
    pub longtitude: i64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
pub struct PutHomeSDto{
    pub name: String,
    pub address: String,
    pub latitude: i64,
    pub longtitude: i64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
pub struct HomeSDto{
    pub id: String,
    pub name: String,
    pub address: String,
    pub latitude: i64,
    pub longtitude: i64,
    pub substation: String, // id of substation
    pub user: String // id of user
}
