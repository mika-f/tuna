use std::str::FromStr;

use http::{
    uri::{Authority, Scheme},
    Uri,
};

#[derive(Debug)]
pub struct GlobalArgs {
    endpoint: Option<String>,

    no_colors: bool,

    no_deserialize: bool,
}

impl GlobalArgs {
    fn normalize_uri(string: Option<String>) -> Option<String> {
        let uri = match &string {
            Some(endpoint) => endpoint.parse::<Uri>(),
            None => return None,
        };

        let uri = match uri {
            Ok(uri) => uri,
            Err(_) => return None,
        };

        if uri.port_u16().is_none() {
            let authority = uri.authority().unwrap().host();
            let scheme = match Scheme::from_str(uri.scheme_str().unwrap()) {
                Ok(scheme) => scheme,
                Err(_) => return None,
            };
            let port = if scheme == Scheme::HTTP { 80 } else { 443 };
            let uri = Uri::builder()
                .scheme(scheme)
                .authority(
                    Authority::from_str(format!("{}:{}", authority.to_owned(), port).as_str())
                        .unwrap(),
                )
                .path_and_query(uri.path_and_query().unwrap().to_string())
                .build()
                .unwrap();

            return Some(uri.to_string());
        } else {
            return Some(string.unwrap());
        }
    }

    pub fn new(endpoint: Option<String>, no_colors: bool, no_deserialize: bool) -> Self {
        Self {
            endpoint: GlobalArgs::normalize_uri(endpoint),
            no_colors,
            no_deserialize,
        }
    }

    pub fn endpoint(&self) -> Option<&String> {
        return self.endpoint.as_ref();
    }

    pub fn no_colors(&self) -> bool {
        return self.no_colors;
    }

    pub fn no_deserialize(&self) -> bool {
        return self.no_deserialize;
    }
}
