// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::Workbook;

mod common;

// Test case to demonstrate creating a basic file with some numeric cell data.
// This tests also verifies the row span ranges.
fn create_new_xlsx_file(filename: &str) {
    let mut workbook = Workbook::new(filename);
    let worksheet = workbook.add_worksheet();

    worksheet.write_number_only(0, 0, 1.0);
    worksheet.write_number_only(1, 1, 2.0);
    worksheet.write_number_only(2, 2, 3.0);

    worksheet.write_number_only(0, 4, 1.0);
    worksheet.write_number_only(1, 5, 2.0);
    worksheet.write_number_only(2, 6, 3.0);

    workbook.close();
}

#[test]
fn bootstrap05_test_spans() {
    let testcase = "bootstrap05";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}
