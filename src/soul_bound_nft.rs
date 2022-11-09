use crate::prelude::*;

#[derive(Serialize, Deserialize, CandidType, Debug, Clone)]
pub struct SoulBoundNFT {
    pub id: u64,
}

impl SoulBoundNFT {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::io::{BufWriter, Write};
    use std::path::Path;

    #[test]
    fn encode_asset() {
        let image_file = Path::new("assets/SNS_TOKEN_IC_PERS_NOBG.png");
        let image = File::open(image_file).unwrap();
        let mut reader = BufReader::new(image);
        let mut buffer: Vec<u8> = vec![];
        let _ = reader.read_to_end(&mut buffer);

        let path = Path::new("assets/image_as_bytes.txt");
        let file = File::create(path).unwrap();
        let mut write = BufWriter::new(file);
        let mut start = 0;
        let mut end = start + 10000;
        while start < buffer.len() {
            let _ = write.write_fmt(format_args!("{:?}\n", &buffer[start..end]));
            start = end;
            end = start + 225000;
            if end > buffer.len() {
                end = buffer.len();
            }
        }
    }
}
