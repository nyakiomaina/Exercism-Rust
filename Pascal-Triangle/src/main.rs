use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
    error::Error,
    result::Result,
};

fn generate_next_row(prev: &Vec<usize>) -> Vec<usize> {
    let mut next = Vec::with_capacity(prev.len() + 1);
    next.push(1);
    for i in 0..prev.len() - 1 {
        next.push(prev[i] + prev[i + 1]);
    }
    next.push(1);
    next
}

fn color_for_index(idx: usize) -> Color {
    match idx % 6 {
        0 => Color::Red,
        1 => Color::Green,
        2 => Color::Yellow,
        3 => Color::Blue,
        4 => Color::Magenta,
        _ => Color::Cyan,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let max_rows = 15;
    let mut all_rows: Vec<Vec<usize>> = Vec::new();

    all_rows.push(vec![1]);

    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print("=== Pascal's Triangle ===\n\n"),
        ResetColor
    )?;

    for row_index in 1..=max_rows {
        if row_index > 1 {
            let prev_row = &all_rows[row_index - 2];
            let next = generate_next_row(prev_row);
            all_rows.push(next);
        }

        execute!(stdout(), Clear(ClearType::All))?;
        execute!(stdout(), MoveTo(0, 0))?;

        execute!(
            stdout(),
            SetForegroundColor(Color::Cyan),
            Print("=== Pascal's Triangle ===\n\n"),
            ResetColor
        )?;

        for (i, row) in all_rows.iter().enumerate() {
            let spacing = " ".repeat(max_rows - i);
            execute!(stdout(), Print(&spacing))?;

            let row_color = color_for_index(i);
            execute!(stdout(), SetForegroundColor(row_color))?;

            for val in row {
                execute!(stdout(), Print(format!("{} ", val)))?;
            }

            execute!(stdout(), ResetColor, Print("\n"))?;
        }

        stdout().flush()?;
        thread::sleep(Duration::from_millis(400));
    }

    execute!(
        stdout(),
        Print("\n"),
        SetForegroundColor(Color::Cyan),
        Print("--- The End ---\n"),
        ResetColor
    )?;

    Ok(())
}
