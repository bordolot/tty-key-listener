use tty_key_listener::TtyListener;
use tty_key_listener::termios;

fn main() -> std::io::Result<()> {
    let mut gti = termios::Gti::new()?;
    let mut lis = TtyListener::new()?;

    gti.turn_terminal_raw_mode_on()?;

    lis.bind_terminal_with_epoll()?;
    if lis.wait_for_next_event()? > 0 {
        lis.read_events()?;
    }
    gti.turn_terminal_canon_mode_on()?;
    Ok(())
}
