// struct ByteLocation<'a> {
//     line_number: usize,
//     column_number: usize,
//     line: &'a str
// }

pub fn format<'a>(source: &'a [u8], index: usize, message: &'a str) -> String {
    format!("{}\n{}", message, get_location(source, index))
}

fn get_location(source: &[u8], index: usize) -> String {
    if index > source.len() {
        panic!("index out of bounds");
    }
    let mut i: usize = 0;
    let mut line_number: usize = 1;
    let mut line_start: usize = 0;
    while i != index  {
        if source[i] == b'\n' {
            line_number += 1;
            line_start = i;
        }
        i += 1;
    }
    let column = index - line_start + 1;
    let line_end = index_of(source, b'\n', index).unwrap_or(source.len());
    format!(
        "Line number: {}, Column number: {}\nLine: {}\n{:>4$}",
        line_number,
        column,
        std::str::from_utf8(&source[line_start .. line_end]).unwrap(),
        '^',
        column + 6 // compensate for "Line: " prefix on previous line
    )
}

fn index_of(source: &[u8], byte: u8, mut offset: usize) -> Result<usize, &'static str> {
    while offset != source.len() {
        if source[offset] == byte {
            return Ok(offset);
        }
        offset += 1;
    }
    Err("Not found")
}