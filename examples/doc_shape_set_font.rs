// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! This example demonstrates adding a Textbox shape and setting some of the
//! font properties.

use rust_xlsxwriter::{Shape, ShapeFont, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Create a textbox shape with formatting.
    let textbox = Shape::textbox().set_text("This is some text").set_font(
        &ShapeFont::new()
            .set_bold()
            .set_italic()
            .set_name("American Typewriter")
            .set_color("#0000FF")
            .set_size(15),
    );

    // Insert a textbox in a cell.
    worksheet.insert_shape(1, 1, &textbox)?;

    // Save the file to disk.
    workbook.save("shape.xlsx")?;

    Ok(())
}
