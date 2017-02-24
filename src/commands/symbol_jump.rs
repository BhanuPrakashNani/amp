use errors::*;
use commands::{self, Result};
use models::application::{Application, Mode};
use models::application::modes::SearchSelectMode;

pub fn jump_to_selected_symbol(app: &mut Application) -> Result {
    if let Mode::SymbolJump(ref mut mode) = app.mode {
        let buffer = app.workspace.current_buffer().ok_or(BUFFER_MISSING)?;
        let position = mode
            .selection()
            .ok_or("Couldn't find a position for the selected symbol")?
            .position;

        if !buffer.cursor.move_to(position) {
            bail!("Couldn't move to the selected symbol's position");
        }
    }

    commands::view::scroll_cursor_to_center(app)?;
    commands::application::switch_to_normal_mode(app)?;

    Ok(())
}

pub fn search(app: &mut Application) -> Result {
    if let Mode::SymbolJump(ref mut mode) = app.mode {
        mode.search();
    } else {
        bail!("Can't search symbols outside of symbol jump mode");
    }

    Ok(())
}

pub fn select_next_symbol(app: &mut Application) -> Result {
    if let Mode::SymbolJump(ref mut mode) = app.mode {
        mode.select_next();
    } else {
        bail!("Can't change symbol selection outside of symbol jump mode");
    }

    Ok(())
}

pub fn select_previous_symbol(app: &mut Application) -> Result {
    if let Mode::SymbolJump(ref mut mode) = app.mode {
        mode.select_previous();
    } else {
        bail!("Can't change symbol selection outside of symbol jump mode");
    }

    Ok(())
}

pub fn enable_insert(app: &mut Application) -> Result {
    if let Mode::SymbolJump(ref mut mode) = app.mode {
        mode.set_insert_mode(true);
    } else {
        bail!("Can't change symbol search insert state outside of symbol jump mode");
    }

    Ok(())
}

pub fn disable_insert(app: &mut Application) -> Result {
    if let Mode::SymbolJump(ref mut mode) = app.mode {
        mode.set_insert_mode(false);
    } else {
        bail!("Can't change symbol search insert state outside of symbol jump mode");
    }

    Ok(())
}
