mod base64;

fn main() {
    println!("Hello, world! {}", base64::encode(&"BC".to_string()));
}
