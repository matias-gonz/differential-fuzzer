#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 1024 {
        return;
    }

    let parsed_with_csv_parser = csv_parser::parse(data.to_vec()).unwrap();

    let parsed_with_quick_csv = quick_csv::Csv::from_string(std::str::from_utf8(data).unwrap());

    let i = 0;
    for row in parsed_with_quick_csv.into_iter() {
        let len_quick_csv = row.unwrap().bytes_columns().len();
        let len_csv_parser = parsed_with_csv_parser[i].len();
        if len_quick_csv != len_csv_parser {
            panic!(
                "len_quick_csv: {}, len_csv_parser: {}",
                len_quick_csv, len_csv_parser
            );
        }
    }
});
