// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! A chart example demonstrating setting the chart title.

use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 50)?;
    worksheet.write(1, 0, 30)?;
    worksheet.write(2, 0, 40)?;

    // Create a new chart.
    let mut chart = Chart::new(ChartType::Column);

    // Add a data series using Excel formula syntax to describe the range.
    chart.add_series().set_values("Sheet1!$A$1:$A$3");

    // Set the chart title.
    chart.title().set_name("This is the chart title");

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
