use zktls_program_core::GuestInput;

use crate::request;

pub fn entry(input: &[u8]) -> Vec<u8> {
    let input: GuestInput = ciborium::from_reader(input).expect("Failed to parse input from cbor");

    entry_input(input)
}

pub fn entry_input(input: GuestInput) -> Vec<u8> {
    let res = request::execute(input.request, input.response);

    let mut res_bytes = Vec::new();

    ciborium::into_writer(&res, &mut res_bytes).unwrap();

    res_bytes
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_entry() {}
}
