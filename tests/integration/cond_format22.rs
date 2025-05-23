// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{ConditionalFormatCell, ConditionalFormatCellRule, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, 10)?;
    worksheet.write(1, 0, 20)?;
    worksheet.write(2, 0, 30)?;
    worksheet.write(3, 0, 40)?;

    let conditional_format =
        ConditionalFormatCell::new().set_rule(ConditionalFormatCellRule::GreaterThan(4));

    worksheet.add_conditional_format(3, 0, 3, 0, &conditional_format)?;

    let conditional_format =
        ConditionalFormatCell::new().set_rule(ConditionalFormatCellRule::GreaterThan(1));

    worksheet.add_conditional_format(0, 0, 0, 0, &conditional_format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_cond_format22() {
    let test_runner = common::TestRunner::new()
        .set_name("cond_format22")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
