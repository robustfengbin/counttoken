#[derive(Debug)]
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