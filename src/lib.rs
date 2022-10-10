
pub struct LinodeAPI {
    pub access_token: String,
}

impl LinodeAPI {
    pub fn new<S>(access_token: S) -> LinodeAPI
    where
         S: Into<String>,
     {
         LinodeAPI {
             access_token: access_token.into(),
         }
     }

     pub fn get_account(&self) {
        println!("in get_account");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_account();
        assert_eq!(result, 4);
    }
}
