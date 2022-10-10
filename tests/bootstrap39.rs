// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};
use std::collections::HashSet;

mod common;

// Test case to demonstrate writing a future function, with explicit xlfn.
fn create_new_xlsx_file_a(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);
    let worksheet = workbook.add_worksheet();

    worksheet.write_formula_only(0, 0, "=_xlfn.ISOWEEKNUM(1)")?;
    worksheet.set_formula_result(0, 0, "52");

    workbook.close()?;

    Ok(())
}

// Test case to demonstrate writing a future function, with implicit xlfn.
fn create_new_xlsx_file_b(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);
    let worksheet = workbook.add_worksheet();

    worksheet.use_future_functions(true);
    worksheet.write_formula_only(0, 0, "=ISOWEEKNUM(1)")?;
    worksheet.set_formula_result(0, 0, "52");

    workbook.close()?;

    Ok(())
}

#[test]
fn bootstrap39_future_function_a() {
    let testcase = "bootstrap39";

    let mut ignore_files: HashSet<&str> = HashSet::new();
    ignore_files.insert("xl/calcChain.xml");
    ignore_files.insert("[Content_Types].xml");
    ignore_files.insert("xl/_rels/workbook.xml.rels");

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames_unique(testcase, 'a');
    _ = create_new_xlsx_file_a(&xlsxwriter_file);
    common::assert_eq_most(&excel_file, &xlsxwriter_file, &ignore_files);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}

#[test]
fn bootstrap39_future_function_b() {
    let testcase = "bootstrap39";

    let mut ignore_files: HashSet<&str> = HashSet::new();
    ignore_files.insert("xl/calcChain.xml");
    ignore_files.insert("[Content_Types].xml");
    ignore_files.insert("xl/_rels/workbook.xml.rels");

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames_unique(testcase, 'b');
    _ = create_new_xlsx_file_b(&xlsxwriter_file);
    common::assert_eq_most(&excel_file, &xlsxwriter_file, &ignore_files);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}
