// use bcrypt::{DEFAULT_COST, hash, verify};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
//use rustc_hex::{ToHex,FromHex};
pub struct PasswordEncoder {}

impl PasswordEncoder {
    pub fn encode(raw_password: &str) -> String {
        //hash(raw_password,DEFAULT_COST).unwrap()
        // create a SHA3-256 object
    let mut hasher = Sha3::sha3_256();

    // write input message
    hasher.input_str(raw_password);

    // read hash digest
     hasher.result_str().to_uppercase()

      }
      pub fn verify(password: &str, raw_password: &str) -> bool {
        // let mut hasher = Sha3::sha3_256();

        // // write input message
        // hasher.input_str(raw_password);
    
        // // read hash digest
        // let pas22 = hasher.result_str().to_lowercase();
        // dbg!(&pas22);
        // let pas2=&password;

        let mut hasher2 = Sha3::sha3_256();

        // write input message
        hasher2.input_str(raw_password);
    
        // read hash digest
        let pas33 = hasher2.result_str().to_lowercase();
        dbg!(&pas33);
        let pas3=&pas33;
        dbg!(format!("raw_password {} pas3 {}  ",raw_password ,pas3)  );
        dbg!(format!("password {}   tosaveUp {} ",raw_password ,pas3)  );
        dbg!(pas3);

        password == pas3

      }

    // pub fn encode2(raw_password: &str) -> String {
    //   hash(raw_password,DEFAULT_COST).unwrap()
    // }
    // pub fn verify2(password: &str, raw_password: &str) -> bool {
    //     verify(raw_password, password).unwrap()
    // }
}

#[cfg(test)]
mod test {
    use crate::util::password_encoder::PasswordEncoder;

    #[test]
    fn test_encode() {
        let s = PasswordEncoder::encode("123456");
        println!("{}", s);
        assert_eq!(
            PasswordEncoder::encode("123456"),
            PasswordEncoder::encode("123456")
        )
    }

    #[test]
    fn test_verify() {
        let password = "12345";
        let raw_password = "12345";

        assert!(PasswordEncoder::verify(password, raw_password));

        let encode_password = PasswordEncoder::encode(password);
        assert!(PasswordEncoder::verify(&encode_password, password));
    }
}
