// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with user defined column.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);
    let worksheet = workbook.add_worksheet();
    let format1 = Format::new().set_bold();
    let format2 = Format::new().set_italic();
    let format3 = Format::new().set_bold().set_italic();

    worksheet.set_column_format(0, &format1)?;
    worksheet.set_column_format(1, &format2)?;
    worksheet.set_column_format(2, &format1)?;

    worksheet.set_column_format(3, &format1)?;
    worksheet.set_column_width(3, 5.86)?;

    worksheet.set_column_format(4, &format1)?;
    worksheet.set_column_format(5, &format1)?;
    worksheet.set_column_format(6, &format1)?;
    worksheet.set_column_format(7, &format1)?;
    worksheet.set_column_width(4, 6.43)?;
    worksheet.set_column_width(5, 6.43)?;
    worksheet.set_column_width(6, 6.43)?;
    worksheet.set_column_width(7, 6.43)?;

    worksheet.set_column_format(8, &format2)?;
    worksheet.set_column_format(5, &format3)?;

    workbook.close()?;

    Ok(())
}

#[test]
fn bootstrap28_set_column_contiguous_formats() {
    let testcase = "bootstrap28";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    _ = create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}