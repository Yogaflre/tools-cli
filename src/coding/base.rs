use data_encoding::{BASE32, BASE64, HEXUPPER};

pub enum BaseType {
    Base64,
    Base32,
    Base16,
}

pub fn convert(text: &str, base_type: BaseType, encode: bool) -> String {
    if encode {
        return base_encode(text, base_type);
    } else {
        return base_decode(text, base_type);
    }
}

fn base_encode(text: &str, base_type: BaseType) -> String {
    let bytes = text.as_bytes();
    return match base_type {
        BaseType::Base64 => BASE64.encode(bytes),
        BaseType::Base32 => BASE32.encode(bytes),
        BaseType::Base16 => HEXUPPER.encode(bytes),
    };
}

fn base_decode(text: &str, base_type: BaseType) -> String {
    let bytes = text.as_bytes();
    return match base_type {
        BaseType::Base64 => BASE64.decode(bytes),
        BaseType::Base32 => BASE32.decode(bytes),
        BaseType::Base16 => HEXUPPER.decode(&bytes.to_ascii_uppercase()),
    }
    .map(|vector| String::from_utf8(vector).unwrap())
    .unwrap();
}
