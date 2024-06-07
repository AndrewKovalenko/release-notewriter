use core::str;
use std::{error::Error, usize};

type IPAddress = [u8; 4];
const IP_ADDRESS_SEPARATOR: &str = ".";
const NUMBER_OF_BYTES_IN_ADDRESS: usize = 4;

pub fn parse_server_adderss(address: &str) -> Result<IPAddress, Box<dyn Error>> {
    let address_byte_strings: Vec<String> = address
        .split(IP_ADDRESS_SEPARATOR)
        .map(|s| s.to_string())
        .collect();

    if address_byte_strings.len() != NUMBER_OF_BYTES_IN_ADDRESS {
        return Err(Box::from("Wrong IP address format"));
    }

    let mut result: IPAddress = [0, 0, 0, 0];

    for i in 0..4 {
        if let Ok(address_byte) = address_byte_strings[i].parse::<u8>() {
            result[i] = address_byte;
        } else {
            return Err(Box::from("Wrong IP address format"));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_ip_address_converted_successfuly() {
        let ip_address_string = "127.0.0.1";
        let result = parse_server_adderss(ip_address_string);

        assert!(Result::is_ok(&result));

        let address_bytes = result.unwrap();

        assert_eq!(address_bytes.len(), NUMBER_OF_BYTES_IN_ADDRESS);
    }

    #[test]
    fn missformated_address_string_returns_error() {
        let ip_address_string = "asdf.asdf.0.0.1";
        let result = parse_server_adderss(ip_address_string);

        assert!(Result::is_err(&result));
    }

    #[test]
    fn random_numbers_in_address_string_returns_error() {
        let ip_address_string = "300.0.0.1";
        let result = parse_server_adderss(ip_address_string);

        assert!(Result::is_err(&result));
    }
}
