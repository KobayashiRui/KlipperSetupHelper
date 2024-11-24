use std::env;
fn main(){
  let profile = env::var("PROFILE").unwrap();
  if profile == "release"{
    println!("cargo:rustc-env=RUST_ENV=production")
  }else{
    println!("cargo:rustc-env=RUST_ENV=development")
  }

}