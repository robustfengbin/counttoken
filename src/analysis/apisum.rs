#[derive(Debug)]
//每个token对应的访问量 struct
pub struct TokenApiSum {
    token: String,
    total: u32
}

impl TokenApiSum {
    pub fn new(token:String,total:u32)->TokenApiSum{
        TokenApiSum {
            token:token,
            total:total
        }
    }
}