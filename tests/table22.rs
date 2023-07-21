// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Table, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(1, 10.288)?;
    worksheet.set_column_width(2, 10.288)?;

    let data = [["apple", "pie"], ["pine", "tree"]];
    worksheet.write_row_matrix(1, 1, data)?;

    let mut table = Table::new();
    table.set_header_row(false);

    worksheet.add_table(1, 1, 2, 2, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table22() {
    let test_runner = common::TestRunner::new()
        .set_name("table22")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}