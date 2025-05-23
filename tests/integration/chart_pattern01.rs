// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    for row_num in 0..3 {
        for col_num in 0..8 {
            worksheet.write_number(row_num as u32, col_num as u16, 2)?;
        }
    }

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(86421504, 86423040);

    chart.add_series().set_values("=Sheet1!$A$1:$A$3");
    chart.add_series().set_values("=Sheet1!$B$1:$B$3");
    chart.add_series().set_values("=Sheet1!$C$1:$C$3");
    chart.add_series().set_values("=Sheet1!$D$1:$D$3");
    chart.add_series().set_values("=Sheet1!$E$1:$E$3");
    chart.add_series().set_values("=Sheet1!$F$1:$F$3");
    chart.add_series().set_values("=Sheet1!$G$1:$G$3");
    chart.add_series().set_values("=Sheet1!$H$1:$H$3");

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_pattern01() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_pattern01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
