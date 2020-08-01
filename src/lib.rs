mod gempa_wrapper;

#[cfg(test)]
mod tests {
    // use crate::gempa_wrapper;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // fn get_data() {
    //     let url = String::from("https://data.bmkg.go.id/autogempa.xml");
    //     let result = [[
    //         ("Tanggal".to_string(), "30-Jul-20".to_string()),
    //         ("Jam".to_string(), "09:51:20 WIB".to_string()),
    //         ("coordinates".to_string(), "131.11,-3.43".to_string()),
    //         ("Lintang".to_string(), "3.43 LS".to_string()),
    //         ("Bujur".to_string(), "131.11 BT".to_string()),
    //         ("Magnitude".to_string(), "5.4 SR".to_string()),
    //         ("Kedalaman".to_string(), "15 Km".to_string()),
    //         ("_symbol".to_string(), "imagesSWF/m2b.swf".to_string()),
    //         (
    //             "Wilayah1".to_string(),
    //             "77 km Tenggara SERAMBAGIANTIMUR-MALUKU".to_string(),
    //         ),
    //         (
    //             "Wilayah2".to_string(),
    //             "215 km BaratDaya FAKFAK-PAPUABRT".to_string(),
    //         ),
    //         (
    //             "Wilayah3".to_string(),
    //             "218 km BaratDaya SORONGSELATAN-PAPUABRT".to_string(),
    //         ),
    //         (
    //             "Wilayah4".to_string(),
    //             "325 km TimurLaut AMBON-MALUKU".to_string(),
    //         ),
    //         (
    //             "Wilayah5".to_string(),
    //             "2715 km TimurLaut JAKARTA-INDONESIA".to_string(),
    //         ),
    //         (
    //             "Potensi".to_string(),
    //             "tidak berpotensi TSUNAMI".to_string(),
    //         ),
    //     ]];

    //     let x = gempa_wrapper::get_data(&url).unwrap();

    //     assert_eq!(x, result);
    // }
}
