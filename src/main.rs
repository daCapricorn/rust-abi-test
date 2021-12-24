fn main() {
}

#[cfg(test)]
mod tests {
    use ethabi::Uint;

    #[test]
    fn test_simple_decode() {
        let data = "0x5eac623900000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000001ae81747fd2d50ae0c7c07716c8b2f873a1c5e10471b077ec6e5fbcd23467e12e";
        // println!("{}", &data[2..]);

        let data = hex::decode(&data[10..]);
        let data = data.unwrap();

        let result = ethabi::decode(&[
            ethabi::ParamType::Array(Box::from(ethabi::ParamType::Uint(256)))
        ], &data).unwrap();

        let result = result[0].clone().into_array().unwrap()[0].clone().into_uint().unwrap();
        println!("{:?}", result);
        let expected = Uint::from(&hex::decode("ae81747fd2d50ae0c7c07716c8b2f873a1c5e10471b077ec6e5fbcd23467e12e").unwrap()[..]);
        println!("{:?}", expected);

        assert_eq!(result, expected);
    }
}
