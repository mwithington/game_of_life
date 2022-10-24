

use std::time::Duration;
use std::{vec, io, thread};
use std::io::{stdin, stdout, Stdout};

use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use crossterm::{execute, terminal};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use rand::{thread_rng, Rng};
use tui::Terminal;
use tui::backend::{ CrosstermBackend };
use tui::layout::Constraint;
use tui::style::{ Style, Modifier};
use tui::symbols::Marker;
use tui::text::{Spans, Span};
use tui::widgets::canvas::{Canvas, MapResolution, Map, Line, Rectangle, Points, self};
use tui::widgets::{Block, Borders};


fn main() -> Result<(), io::Error> {

    println!("Welcome to conways game of life!");
    println!("Please create your world by entering the number of");

    println!("Rows: ");
    let mut row_input = String::new();
    stdin().read_line(&mut row_input).unwrap();
    row_input.remove(row_input.len() - 1);
    let rows: i32 = row_input.parse::<i32>().unwrap();


    println!("And Columns: ");
    let mut col_input = String::new();
    stdin().read_line(&mut col_input).unwrap();
    col_input.remove(col_input.len() - 1);
    let cols: i32 = col_input.parse::<i32>().unwrap();

    
    // Draw new terminal to play game
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    

    let mut world = create_world(rows, cols);
    let mut stopper = 0;
    loop {
        if stopper == 20 {
            break;
        }
        stopper +=1;
        // , &mut window
        run_game(&mut world, &mut terminal);
        thread::sleep(Duration::from_millis(10));
    }


    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    //

    
    Ok(())
}

fn create_world(rows: i32, cols: i32) -> Vec<Vec<i32>>{
    let mut world: Vec<Vec<i32>> = vec![];
     for row in 0..rows {
        let mut col: Vec<i32> = vec![];
        for x in 0..cols {
            col.push(thread_rng().gen::<i32>().abs() % 2);
        }
        // println!("{:?}", col);
        world.push(col.clone());
     }
     return world;
}

// , window: &mut Window
fn run_game (world: &mut Vec<Vec<i32>>, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let snapshot = world.clone();
    // println!("new lap");
    for y_coord in 0..world.len() {
        let y = snapshot.get(y_coord).unwrap();
        for x_coord in 0..(y.len()-1) {
            let status = get_status(x_coord, y_coord, snapshot.clone());
            if status {
                world[y_coord][x_coord] = 1;
            } else {
                world[y_coord][x_coord] = 0;
            }
        }
        // println!("{:?}", world[y_coord]);
    }
    // , window
    draw_cycle(&world, terminal);
}

// , window: &mut Window
fn draw_cycle(snapshot: &Vec<Vec<i32>>, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let mut needs_dawing: Vec<(f64, f64)> = vec![];
    let mut needs_blanking: Vec<(f64, f64)> = vec![];
    for y_coord in 0..(snapshot.len()-1) {
        let y = snapshot.get(y_coord).unwrap();
        for x_coord in 0..(y.len()-1) {
           let is_live = snapshot[y_coord][x_coord] == 1;
           if is_live {
                // let point = Point::new(x_coord as i32, y_coord as i32);
                // window.draw_point(point);
               needs_dawing.push((x_coord as f64, y_coord as f64));
           } else {
               needs_blanking.push((x_coord as f64, y_coord as f64))
           }
        }
    }

    // println!("{:?}", needs_dawing.as_slice())
    // terminal.draw(|f| {
    //     let size = f.size();
    //     let canvas = Canvas::default()
    //     .marker(Marker::Block)
    //     .block(Block::default().title("Game of life").borders(Borders::NONE))
    //     .x_bounds([0.0, 160.0])
    //     .y_bounds([0.0, 90.0])
    //     .paint(|ctx| {
    //         ctx.draw(&Points {
    //             coords: needs_dawing.as_slice(),
    //             color: Color::LightRed
    //         });
    //         ctx.draw(&Points {
    //             coords: needs_blanking.as_slice(),
    //             color: Color::Black
    //         })
    //     });
    //     f.render_widget(canvas, size);
    // }).unwrap();
}

fn get_status (x_coord: usize, y_coord: usize, world: Vec<Vec<i32>>) -> bool {

    let mut live_cells: i16 = 0;
    let mut status = false;
    let y = world.get(y_coord).unwrap();

    // v
    if y_coord < world.len() - 1 && *world.get(y_coord+1).unwrap().get(x_coord).unwrap() == 1 {
        live_cells += 1;
    }
    // ^
    if y_coord > 0 && *world.get(y_coord - 1).unwrap().get(x_coord).unwrap() == 1 {
        live_cells += 1;
    }

    // check_rights()
    if x_coord < y.len() {
        // >
        if *y.get(x_coord+1).unwrap() == 1 {
            live_cells += 1;
        }
        // > && v
        if y_coord < world.len() - 1 && *world.get(y_coord+1).unwrap().get(x_coord+1).unwrap() == 1 {
            live_cells += 1;
        }
        // > && ^
        if y_coord > 0 && *world.get(y_coord - 1).unwrap().get(x_coord+1).unwrap() == 1 {
            live_cells += 1;
        }
    }

    // check_lefts()
    if x_coord > 0 {
        // <
        if *y.get(x_coord-1).unwrap() == 1 {
            live_cells += 1;
        }
        // < && v
        if y_coord < world.len() - 1 && *world.get(y_coord+1).unwrap().get(x_coord-1).unwrap() == 1 {
            live_cells += 1;
        }
        // < && ^
        if y_coord > 0 && *world.get(y_coord - 1).unwrap().get(x_coord-1).unwrap() == 1 {
            live_cells += 1;
        }
    }
    if live_cells < 2 || live_cells > 3 {
    } else {
        status = true;
    }
    return status;
}