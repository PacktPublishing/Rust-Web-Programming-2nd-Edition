use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct HttpFrame {
    pub header: Header,
    pub body: Body
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub method: String,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub ticker: String,
    pub amount: f32,
}