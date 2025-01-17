// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates some common formula syntax errors.

use rust_xlsxwriter::{Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // OK.
    worksheet.write_formula(0, 0, "=SUM(1, 2, 3)")?;

    // Semi-colon separator. Causes Excel error on file opening.
    worksheet.write_formula(1, 0, "=SUM(1; 2; 3)")?;

    // French function name. Causes Excel error on file opening.
    worksheet.write_formula(2, 0, "=SOMME(1, 2, 3)")?;

    workbook.save("formula.xlsx")?;

    Ok(())
}
