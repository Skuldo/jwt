use std::str;
use crate::base64::base64_pad;

pub enum Sections {
    Header,
    Payload,
    Signature,
    Full
}

fn jwt_pad(jwt: String, section: Sections) -> String {

    let mut parsed_section = String::new();
    if jwt.starts_with("eyJ") {
        let jwt_sections: Vec<&str> = jwt.split(".").collect();
        match section {
            Sections::Header => parsed_section = String::from(jwt_sections[0]),
            Sections::Payload => parsed_section = String::from(jwt_sections[1]),
            Sections::Signature => parsed_section = String::from(jwt_sections[2]),
            Sections::Full => parsed_section = String::from(&jwt)
        }
        //jwt_body = String::from(jwt_sections[1]);
    }
    else {
        eprintln!("Invalid JSON Web Token provided");
    }




    while parsed_section.len() % 4 != 0 {
        parsed_section = base64_pad(parsed_section);
    }
    //println!("[Length: {}, Parsed: {}", parsed_section.len(), parsed_section);

    parsed_section = str::replace(parsed_section.as_str(), "-", "+");
    //println!("{}", parsed_section); //
    parsed_section = str::replace(parsed_section.as_str(), "_", "/");
    //println!("{}", parsed_section); //

    parsed_section

}

pub fn jwt_decode(jwt: String, section: Sections) -> String {
    let jwt_padded = jwt_pad(jwt, section);
    let jwt_decoded_bytes = base64::decode(&jwt_padded).unwrap();
    let jwt_decoded = str::from_utf8(&jwt_decoded_bytes).unwrap();

    String::from(jwt_decoded)
}