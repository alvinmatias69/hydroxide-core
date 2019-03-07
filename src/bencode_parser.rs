use super::meta_info::*;
use super::{
    BencodeData,
    bencode
};

trait BencodeParser {
    fn init(bencoded_data: &String) -> Self;
    fn parse() -> MetaInfo;
}

struct Parser {
    bencoded_data: BencodeData
}

impl BencodeParser for Parser {
    fn init(bencoded_data: &String) -> Parser {
        let parser = Parser{ bencoded_data: bencode::decode(bencoded_data) };
        parser
    }

    fn parse() -> MetaInfo {
        let list_string: Vec<String> = Vec::new();
        MetaInfo {
            announce: String::from(""),
            announce_list: vec![list_string],
            comment: String::from(""),
            created_by: String::from(""),
            creation_date: 0,
            encoding: String::from(""),
            pieces: String::from(""),
            pieces_length: 0,
            private: false,
            info: InfoType::unset,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_bencode_string_successfully() {
        let data = String::from("l5:list15:list2e");
        let parser = Parser::init(&data);
        if let BencodeData::List(_) = parser.bencoded_data {
        } else {
            panic!("Should be successfully return as bencoded list");
        }
    }

    #[test]
    fn should_have_correct_announcement() {

    }
}