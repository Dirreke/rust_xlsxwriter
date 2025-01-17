// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! An example of formatting the chart hole size for doughnut charts.

use rust_xlsxwriter::{Chart, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 10)?;
    worksheet.write(1, 0, 40)?;
    worksheet.write(2, 0, 50)?;

    // Create a new chart.
    let mut chart = Chart::new_doughnut();

    // Add a data series with formatting.
    chart.add_series().set_values("Sheet1!$A$1:$A$3");

    // Set the home size of the chart.
    chart.set_hole_size(80);

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
