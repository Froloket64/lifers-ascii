// Conway's Game of Life implementation

use lifers::prelude::{count_neighbors, Automaton, RenderCell};
use lifers_ascii::AsciiFrontend;

const GENERATION_LIMIT: u32 = 10;
const GRID_SIZE: (usize, usize) = (10, 10);

// NOTE: The only reason this is required is because one can't implement foreign
// traits on primitives like `bool` :(
#[derive(Default)]
struct Cell {
    is_alive: bool,
}

impl RenderCell<String> for Cell {
    fn render_cell(&self) -> String {
        match self.is_alive {
            true => "██",
            false => "··",
        }
        .into()
    }
}

fn main() {
    let spawn = [(2, 0), (2, 1), (3, 1), (1, 2), (2, 2)];
    let game = Automaton::build(GRID_SIZE)
        .generations(GENERATION_LIMIT)
        .init(|pos| Cell {
            is_alive: spawn.contains(&pos),
        })
        // Compute number of neighbors
        .map(|(x, y), _, cells| count_neighbors(cells, (x, y), 1, |c| c.is_alive))
        // Change state depending on the number of neighbors
        .run(|(_, _), cell, neighbors_n| match cell.is_alive {
            true => Cell {
                is_alive: (2..=3).contains(neighbors_n),
            },
            false => Cell {
                is_alive: *neighbors_n == 3,
            },
        });

    let mut ascii_frontend = AsciiFrontend::new(game);

    while let Some(s) = ascii_frontend.render_next() {
        println!("{s}");
    }
}
