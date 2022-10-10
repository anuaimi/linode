
use linode::LinodeAPI;

fn main() {
  
  let access_token= "at-184832";
  let api = LinodeAPI::new(access_token);

  api.get_account();

}