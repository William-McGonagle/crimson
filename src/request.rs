use std::collections::HashMap;

pub struct Request {

    pub method:String,
    pub version:String,
    pub path:String,
    pub body:String,
    pub headers:HashMap<String, String>

}

impl Request {
    
    #[allow(dead_code)]
    pub fn request_full(self) -> String {

        return format!(
            "{} {} {}\r\n{}\r\n{}",
            self.method,
            self.path,
            self.version,
            Request::request_headers(self.headers),
            self.body
        );

    }
    
    #[allow(dead_code)]
    pub fn request_headers(headers:HashMap<String, String>) -> String {

        let mut output = "".to_owned();

        for (key, value) in headers {
        
            output.push_str(&key);
            output.push_str(": ");
            output.push_str(&value);
            output.push_str("\r\n");
        
        }

        return output;

    }

}

pub fn parse_request(input:[u8; 1024]) -> Request {

    let mut state = 0;
    let mut index = 0;
    let chars = input;

    let mut request_method:String = "".to_owned();
    let mut request_path:String = "".to_owned();
    let mut request_version:String = "".to_owned();

    let mut request_headers:HashMap<String, String> = HashMap::new();

    let mut request_body:String = "".to_owned();

    let mut error_response:String = "".to_owned();

    let mut current_header_label:String = "".to_owned();
    let mut current_header_value:String = "".to_owned();

    loop {

        match state {

            // HTTP-Method State
            0 => {

                match chars[index] {

                    0x20 => { state = 1; } // Set State to Path State if Character is Space
                    _ => { request_method.push(chars[index] as char); } // Add current char to `request_method`

                }

                index = index + 1;

            },

            // Path State
            1 => {

                match chars[index] {

                    0x20 => { state = 2; }, // Set State to Version State if Character is Space
                    _ => { request_path.push(chars[index] as char); }, // Add current char to `request_path`

                }

                index = index + 1;

            },

            // HTTP-Version State
            2 => {

                match chars[index] {

                    0x0D => { state = 3; }, // Set State to Newline State if Carriage Return 
                    0x0A => { state = 4; }, // Set State to Header State if Newline
                    _ => { request_version.push(chars[index] as char); }, // Add current char to `request_version`

                }

                index = index + 1;

            },

            // Newline Intermediate (for windows and linux line-ending support)
            3 => {

                match chars[index] {

                    0x0A => { state = 4; }, // Set State to Header State if Newline
                    _ => { error_response = format!("Improper Break from Head to Header. Expected '10' not '{}'.", chars[index]); break; } // Log Error if Improper Request is Sent

                }

                index = index + 1;

            },

            // Read the HTTP-Header Labels
            4 => {

                match chars[index] {

                    0x3A => { state = 5; },
                    0x0D => { 

                        state = 7;

                    },
                    0x0A => {

                        state = 8;

                    },
                    _ => { current_header_label.push(chars[index] as char); }

                }

                index = index + 1;

            },

            // Read the HTTP-Header Value
            5 => {

                match chars[index] {

                    0x0D => { 

                        // If Carriage Return, Set State to Newline State
                    
                        request_headers.insert(current_header_label, current_header_value);

                        current_header_label = "".to_owned();
                        current_header_value = "".to_owned();

                        state = 6; 
                    
                    },
                    0x0A => { 
                        
                        // If Newline, Set State to Header State
                        
                        request_headers.insert(current_header_label, current_header_value);

                        current_header_label = "".to_owned();
                        current_header_value = "".to_owned();

                        state = 4; 
                    
                    }, 
                    0x20 => { 
                        
                        // Check if Header has any Value Yet
                        if current_header_value.len() > 0 { 

                            // If so, Add the Space (Gets Rid of Padding)
                            current_header_value.push(' ');

                        }
                    
                    },
                    _ => { current_header_value.push(chars[index] as char); } // Add Current to Stack

                }
                
                index = index + 1;

            },

            // Newline Intermediate for Headers to Headers (for windows and linux line-ending support)
            6 => {

                match chars[index] {

                    0x0A => { state = 4; }, // Set State to Header State if Newline
                    _ => { error_response = format!("Improper Break from Header to Header. Expected '10' not '{}'.", chars[index]); break; } // Log Error if Improper Request is Sent

                }

                index = index + 1;

            },

            // Newline Intermediate for Headers to Body (for windows and linux line-ending support)
            7 => {

                match chars[index] {

                    0x0A => { state = 8; }, // Set State to Body State if Newline
                    _ => { error_response = format!("Improper Break from Header to Body. Expected '10' not '{}'.", chars[index]); break; } // Log Error if Improper Request is Sent

                }

                index = index + 1;

            },

            // Body State
            8 => {

                if index == chars.len() { 
                    
                    break; 
                
                }

                request_body.push(chars[index] as char);
                index = index + 1;

            },

            // If State is Out of Bounds
            _ => {

                break;

            }

        }

    }

    if error_response.len() > 0 {

        println!("{}", error_response);

    }

    return Request {

        method: request_method,
        version: request_version,
        path: request_path,
        body: request_body,
        headers: request_headers

    };

} 