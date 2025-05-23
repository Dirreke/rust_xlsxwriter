// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    // Test that control characters and any other single byte characters are
    // handled correctly by the SharedStrings module. We skip chr 34 = " in
    // this test since it isn't encoded by Excel as &quot;.
    for i in 0u8..128 {
        if i == 34 {
            continue;
        }

        worksheet.write_string(i as u32, 0, (i as char).to_string())?;
    }

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize06() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize06")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
