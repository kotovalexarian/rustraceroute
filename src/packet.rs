use crate::checksum::checksum;

pub struct Packet {
    pub ident: u16,
    pub sequence: u16,
    payload: Vec<u8>,
}

impl Packet {
    pub fn new(ident: u16, sequence: u16) -> Self {
        Self {
            ident,
            sequence,
            payload: vec![],
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        result.push(8); // type
        result.push(0); // code
        result.push(0); // checksum
        result.push(0); // checksum
        result.push(((self.ident & 0xFF00) >> 8) as u8);
        result.push((self.ident & 0x00FF) as u8);
        result.push(((self.sequence & 0xFF00) >> 8) as u8);
        result.push((self.sequence & 0x00FF) as u8);
        result.append(&mut self.payload.clone());

        let checksum = checksum(&result);

        result[2] = ((checksum & 0xFF00) >> 8) as u8;
        result[3] = (checksum & 0x00FF) as u8;

        result
    }
}

impl Into<Vec<u8>> for Packet {
    fn into(self) -> Vec<u8> {
        self.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id0_seq0_empty_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 255, 0, 0, 0, 0]);
    }

    #[test]
    fn id1_seq0_empty_into_vec_u8() {
        let packet = Packet { ident: 1, sequence: 0, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 254, 0, 1, 0, 0]);
    }

    #[test]
    fn id255_seq0_empty_into_vec_u8() {
        let packet = Packet { ident: 255, sequence: 0, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 0, 0, 255, 0, 0]);
    }

    #[test]
    fn id65535_seq0_empty_into_vec_u8() {
        let packet = Packet { ident: 65535, sequence: 0, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 255, 255, 255, 0, 0]);
    }

    #[test]
    fn id0_seq1_empty_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 1, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 254, 0, 0, 0, 1]);
    }

    #[test]
    fn id0_seq255_empty_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 255, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 0, 0, 0, 0, 255]);
    }

    #[test]
    fn id0_seq65535_empty_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 65535, payload: vec![] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 255, 0, 0, 255, 255]);
    }

    #[test]
    fn id0_seq0_pay00_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![0] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 255, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn id0_seq0_pay01_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![1] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 246, 255, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn id0_seq0_payff_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![255] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 248, 254, 0, 0, 0, 0, 255]);
    }

    #[test]
    fn id0_seq0_pay0000_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![0, 0] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 255, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn id0_seq0_pay0001_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![0, 1] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 254, 0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn id0_seq0_pay00ff_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![0, 255] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 247, 0, 0, 0, 0, 0, 0, 255]);
    }

    #[test]
    fn id0_seq0_pay8080_into_vec_u8() {
        let packet = Packet { ident: 0, sequence: 0, payload: vec![128, 128] };
        let result: Vec<u8> = packet.into();

        assert_eq!(result, &[8, 0, 119, 127, 0, 0, 0, 0, 128, 128]);
    }
}
