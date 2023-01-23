// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{FilterCondition, Workbook, XlsxError};

mod common;

// Test to demonstrate autofilters.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Write the headers.
    worksheet.write_string_only(0, 0, "Region").unwrap();
    worksheet.write_string_only(0, 1, "Item").unwrap();
    worksheet.write_string_only(0, 2, "Volume").unwrap();
    worksheet.write_string_only(0, 3, "Month").unwrap();

    // Write the data used in the autofilter.
    let mut data = common::get_autofilter_data();

    // Create a blank cell for testing.
    data[5].0 = "";

    for (row, data) in data.iter().enumerate() {
        let row = 1 + row as u32;
        worksheet.write_string_only(row, 0, data.0)?;
        worksheet.write_string_only(row, 1, data.1)?;
        worksheet.write_number_only(row, 2, data.2)?;
        worksheet.write_string_only(row, 3, data.3)?;
    }

    worksheet.autofilter(0, 0, 50, 3)?;

    let filter_condition = FilterCondition::new().list_match_blanks();
    worksheet.filter_column(0, &filter_condition)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofilter05() {
    let test_runner = common::TestRunner::new()
        .set_name("autofilter05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
