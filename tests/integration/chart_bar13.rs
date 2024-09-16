// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data = [[1, 2, 3, 4, 5], [2, 4, 6, 8, 10], [3, 6, 9, 12, 15]];
    for (col_num, col_data) in data.iter().enumerate() {
        for (row_num, row_data) in col_data.iter().enumerate() {
            worksheet.write(row_num as u32, col_num as u16, *row_data)?;
        }
    }

    // Cerate a chart.
    let mut chart = Chart::new(ChartType::Bar);
    chart.set_axis_ids(40294272, 40295808);

    chart.add_series().set_values(("Sheet1", 0, 0, 4, 0));
    chart.add_series().set_values(("Sheet1", 0, 1, 4, 1));
    chart.add_series().set_values(("Sheet1", 0, 2, 4, 2));

    // Create a chartsheet and add the chart.
    let chartsheet = workbook.add_chartsheet().set_name("Chart1")?;
    chartsheet.insert_chart(8, 4, &chart)?;

    let _ = workbook.add_worksheet().set_name("Sheet2")?;
    let _ = workbook.add_worksheet().set_name("Sheet3")?;

    let chartsheet = workbook.add_chartsheet().set_name("Chart2")?;
    chart.set_axis_ids(62356096, 62366080);
    chartsheet.insert_chart(0, 0, &chart)?;

    let _ = workbook.add_worksheet().set_name("Sheet4")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_bar13() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_bar13")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
