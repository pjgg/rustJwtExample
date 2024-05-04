use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const TOTAL_ITERATIONS: usize = 1_000_000;
const AMOUNT_THREAD: usize = 32;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: u64,
    exp: u64,
}

fn main() {
    let file_contents =
        fs::read_to_string("/home/pagonzal/Documents/workspace/rustExample/emails.json")
            .unwrap();
    let emails: Vec<String> = serde_json::from_str(&file_contents).unwrap();
    let jwt_secret = std::fs::read_to_string("/home/pagonzal/Documents/workspace/jwtSignExample/src/main/resources/private_key.pem").unwrap();
    let jwt_encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let total_op = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..AMOUNT_THREAD {
        let emails_clone = emails.clone();
        let jwt_encoding_key_clone = jwt_encoding_key.clone();
        let total_op_clone = Arc::clone(&total_op);

        let handle = thread::spawn(move || {
            let jwt_secret_thread = std::fs::read_to_string("/home/pagonzal/Documents/workspace/jwtSignExample/src/main/resources/private_key.pem").unwrap();
            let jwt_decoding_key_thread = DecodingKey::from_secret(jwt_secret_thread.as_bytes());

            for _ in 0..TOTAL_ITERATIONS {
                let idx = generate_random_position(99_999);
                let email = emails_clone[idx].clone();
                let curr_ts = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                let my_claims = Claims {
                    sub: email.clone(),
                    iat: curr_ts,
                    exp: curr_ts + 2 * 60 * 60 * 1000,
                };
                let token = match encode(&Header::default(), &my_claims, &jwt_encoding_key_clone)
                {
                    Ok(t) => t,
                    Err(_) => panic!(),
                };
                println!("Token: {}", token);
                let token_data = match decode::<Claims>(&token, &jwt_decoding_key_thread, &Validation::new(Algorithm::HS256)) {
                    Ok(c) => c,
                    Err(err) => panic!("{}", err.to_string()),
                };
                if token_data.claims.sub != email {
                    panic!("email didn't match");
                }
                let current_total_op = total_op_clone.fetch_add(1, Ordering::Relaxed);
                println!("{}", current_total_op);
                if current_total_op >= TOTAL_ITERATIONS {
                    break;
                }
            }
        });
        handles.push(handle);
    }
    
    let mut start_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    for handle in handles {
        handle.join().unwrap();
    }

    let end_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    let diff = end_ts - start_ts;
    println!("{}", diff);
}

fn generate_random_position(max: usize) -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=max)
}