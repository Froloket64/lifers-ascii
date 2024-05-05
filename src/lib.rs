//! A simple ASCII frontend for [`lifers`]

use lifers::prelude::{Automaton, ExecutionState, RenderCell};

/// The main frontend struct.
pub struct AsciiFrontend<S, D> {
    automaton: Automaton<S, D>,
}

impl<S, D> AsciiFrontend<S, D>
where
    S: RenderCell<String>,
{
    /// Creates a frontend holding an automaton.
    pub fn new(automaton: Automaton<S, D>) -> Self {
        Self { automaton }
    }

    /// Renders the next frame.
    pub fn render_next(&mut self) -> Option<String> {
        match self.automaton.step() {
            ExecutionState::Finished => return None,
            _ => (),
        };

        Some(
            self.automaton
                .cells
                .iter()
                .map(|xs| xs.iter().map(|x| x.render_cell()).collect::<String>())
                // OPTIM: When is this coming to stable??
                // .intersperse("\n".into()).collect()
                .collect::<Box<_>>()
                .join("\n"),
        )
    }
}
