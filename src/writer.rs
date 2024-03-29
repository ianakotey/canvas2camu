use std::io::Error;

use simple_excel_writer::{blank, row, CellValue, Column, Row, Workbook};
use lazy_static::lazy_static;
use regex::Regex;

use crate::config;
use crate::utils::with_temp_dir;

fn create_file(
    file_name : &str, grades : &config::GradeMap, enrollment : &config::EnrollmentData
) -> Result<(), Error> {


    let safe_file_name =  {
        lazy_static! {
            static ref PATH_FIX_RE: Regex = Regex::new(r#"[\\/~!#$%^&*{}<>:?|"-]"#).expect("Ian, fix the regex 🙄🙁😡");
        }

        PATH_FIX_RE.replace_all( file_name, "_") 
    };

    let mut workbook = Workbook::create(&(format!("{}.xlsx", safe_file_name).replace(":", "-")));

    let mut sheet = workbook.create_sheet(config::WRITER_SHEET_NAME);

    // set column width
    sheet.add_column(Column { width : 15.0 });

    sheet.add_column(Column { width : 15.0 });

    sheet.add_column(Column { width : 15.0 });

    sheet.add_column(Column { width : 30.0 });

    sheet.add_column(Column { width : 15.0 });

    sheet.add_column(Column { width : 15.0 });

    // Write header rows to file
    workbook.write_sheet(&mut sheet, |sheet_writer| {

        sheet_writer.append_row(row![
            "StuRollNo",
            "Mark",
            "IsAbs",
            "StuNm",
            "InEligible",
            "rsSts"
        ])?;

        sheet_writer.append_row(row![
            "Roll No",
            "Marks",
            "Is Absent",
            "Student Name",
            "InEligible",
            "Result Status"
        ])?;

        for (email, name, student_id) in enrollment.iter() {

            if !grades.contains_key(email) {
                // Todo: Refactor
                println!("Warning: Student {student_id} with email {email} not found!\n");
                continue;
            }

            let current_grade = grades[email].as_str();

            let (current_grade, current_status) = match grades[email].as_str() {
                "EX" => ("", "Y"),
                "N/A" | "" => ("0.00", "N"),
                _ => (current_grade, "N")
            };

            sheet_writer.append_row(row![
                student_id.as_str(),
                current_grade,
                current_status,
                name.as_str(),
                blank!(2)
            ])?;
        }

        sheet_writer.append_row(row![blank!(6)])?;

        Ok(())
    })?;

    workbook.close().map(|_result| ())
}

/// Creates CAMU-compatible excel files from the class gradebook and enrollment data
///
/// # Arguments
///
/// * `output_dir` - A string slice that holds the output directory
/// * `gradebook` - A reference to a Gradebook struct
/// * `enrollment` - A reference to an EnrollmentData struct

pub(crate) fn create_files(
    output_dir : &str, gradebook : &config::Gradebook, enrollment : &config::EnrollmentData
) -> Result<(), Error> {

    with_temp_dir!(output_dir, {

        for assignment_name in gradebook.keys() {

            create_file(assignment_name, &gradebook[assignment_name].1, enrollment)?;
        }
    });

    Ok(())
}
