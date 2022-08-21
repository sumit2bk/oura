use std::sync::Arc;
use std::time::Duration;

use crate::pipelining::StageReceiver;
use crate::utils::throttle::Throttle;
use crate::utils::Utils;

pub type Error = Box<dyn std::error::Error>;

use crossterm::style::{Color, Print, Stylize};
use crossterm::ExecutableCommand;
use std::io::stdout;

use super::format::*;

pub fn reducer_loop(
    throttle_min_span: Duration,
    terminal_width: Option<u16>,
    input: StageReceiver,
    utils: Arc<Utils>,
) -> Result<(), Error> {
    let mut stdout = stdout();

    let mut throttle = Throttle::new(throttle_min_span);

    stdout.execute(Print(
        "Oura terminal output started, waiting for chain data\n".with(Color::DarkGrey),
    ))?;

    for evt in input.iter() {
        
        let width = match terminal_width {
             Some(w) => w,
             None => crossterm::terminal::size()?.0
        };

        throttle.wait_turn();
        let line = LogLine::new(&evt, width as usize, &utils);

        let result = stdout.execute(Print(line));

        match result {
            Ok(_) => {
                // notify progress to the pipeline
                utils.track_sink_progress(&evt);
            }
            Err(err) => {
                log::error!("error writing to terminal: {}", err);
                return Err(Box::new(err));
            }
        }
    }

    Ok(())
}
