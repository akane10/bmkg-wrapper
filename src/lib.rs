pub mod gempa;

#[cfg(test)]
mod tests {
    use crate::gempa;

    #[test]
    fn get_data() {
        // let result = ("Tanggal".to_string(), "30-Jul-20".to_string());

        let data = gempa::get_data(gempa::Url::Autogempa).unwrap();
        let (key, _) = &data[0][0];

        assert_eq!(key, &"Tanggal".to_string());
    }
}
