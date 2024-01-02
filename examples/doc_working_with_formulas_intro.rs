// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates writing a simple formula.

use rust_xlsxwriter::{Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_formula(0, 0, "=10*B1 + C1")?;

    worksheet.write_number(0, 1, 5)?;
    worksheet.write_number(0, 2, 1)?;

    workbook.save("formula.xlsx")?;

    Ok(())
}
