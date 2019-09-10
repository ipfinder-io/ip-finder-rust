// extern crate hyper;

// use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header};

pub struct Ipfinder {
    // t: Option<String>,
    // b: Option<String>
    token: String,
    base_url:String,
}


// pub struct Data {
//     // t: Option<String>,
//     // b: Option<String>
//     d: String,
//     a: String,
//     c: String
// }
// struct Ipfinder;
// const DEFAULT_BASE_URL: &str = "https://api.ipfinder.io/v1/";

// const DEFAULT_API_TOKEN: &str = "free";

// const FORMAT: &str = "json";

// const STATUS_PATH: &str = "info";

// const RANGES_PATH: &str = "ranges/";

// const FIREWALL_PATH: &str = "firewall/";

// const DOMAIN_PATH: &str = "domain/";

// const DOMAIN_H_PATH: &str = "domainhistory/";

// const DOMAIN_BY_PATH: &str = "domainby/";

static POST_DATA: &str = r#"{"original": "data"}"#;


impl Ipfinder {

  pub fn new(_token:  &str, _base_url:  &str) -> Ipfinder  {


   Ipfinder {
    token: _token.to_string(),
    base_url :_base_url.to_string(),
   }
 }




 pub fn call(mut self, _path: &str, _format: &str)  {

    //self.token = _path.to_string();

    //self.base_url = _format.to_string();

 }

 pub fn authentication(&self)  {
     Ipfinder::call(self,"","json");
 }

 // pub fn get_address_info(_path: &str)  {
 //   Ipfinder::call(_path,"json");
 // }

 // pub fn get_asn(_path: &str)  {
 //   Ipfinder::call(_path,"json");
 // }

 // pub fn get_status(&self)  {
 //   Ipfinder::call("","json");
 // }

 // pub fn get_ranges(_path: &str)  {
 //   Ipfinder::call(_path,"json");
 // }

 // pub fn get_firewall(_path: &str)  {
 //   Ipfinder::call(_path,"json");
 // }

 // pub fn get_domain(_path: &str)  {
 //   Ipfinder::call(_path,"json");
 // }

 // pub fn get_domain_history(_path: &str)  {
 //   Ipfinder::call(_path,"json");
 // }

 // pub fn get_domain_by(_by: &str)  {
 //   Ipfinder::call(_by,"json");
 // }

}
// execute
fn main() {
    let ip = Ipfinder::new("free","sdfsdf");

}
