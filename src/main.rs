use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u128,
    exp: u128,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_contents =
        fs::read_to_string("/home/pagonzal/Documents/workspace/rustExample/emails.json")
            .unwrap();
    let emails: Vec<String> = serde_json::from_str(&file_contents).unwrap();

    let mut i = 1;
    let mut idx = 0;
    let jwt_secret = env::var("JWT_SECRET").expect("$JWT_SECRET is not set");
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let num_iterations = args[1].parse::<i32>().unwrap();
    let mut start_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let validation = Validation::new(Algorithm::HS256);

    loop {
        let email = &emails[idx];
        idx += 1;
        let curr_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        let my_claims = Claims {
            sub: email.to_string(),
            iat: curr_ts,
            exp: curr_ts + 2 * 60 * 60 * 1000,
        };
        
        let token = match encode(&Header::default(), &my_claims, &jwt_encoding_key) {
            Ok(t) => t,
            Err(_) => panic!(),
        };
        println!("Token: {}", token);
        let token_data = match decode::<Claims>(&token, &jwt_decoding_key, &validation) {
            Ok(c) => c,
            Err(err) => panic!("{}", err.to_string()),
        };
        if token_data.claims.sub != email.to_string() {
            panic!("email didn't match");
        }
        if idx >= emails.len() {
            idx = 0;
        }
        i += 1;
        if i > num_iterations {
            break;
        }
    }

    let end_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let diff = end_ts - start_ts;
    println!("{}", diff);
}
