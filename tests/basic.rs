#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use sgm41511::*;

    #[test]
    fn test_get_device_revision() {
        let expectations = [Transaction::write_read(
            SGM41511_ADDR,
            vec![Register::Reg0b as u8],
            vec![0b00010100],
        )];

        let mut i2c = Mock::new(&expectations);

        let mut device = SGM41511::new(i2c.clone());
        let revision = device.get_device_revision().unwrap();

        i2c.done();

        assert_eq!(revision, Some(0));
    }
}
