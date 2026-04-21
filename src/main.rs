//! Todo:
//!  - Implement read of .timer files,
//!  - Implement "except" timer intepretation, including writing of "except" 
//!  arrays in the custom extension of the .timer file types,
//!  - Implement .stamp file writing when scheduling tasks daily,
//!  - Implement .last file writing when completing tasks, to guarantee that 
//!  dynamically scheduled tasks are falling within the schedule format,
//!  - Implement scheduling of the tasks themselves.
mod forecast;

use std::os::unix::io::AsRawFd;

use mio::{Events, Poll, Token, Interest};
use mio::unix::SourceFd;
use nix::sys::signal::{sigprocmask, SigmaskHow, Signal, SigSet};
use nix::sys::signalfd::SignalFd;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// The token associated with either SIGHUP (for reloading cfg), or a terminating signal (SIGTERM, SIGINT).
const SIGNAL_TOKEN: Token = Token(1);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false));
    
    match tracing_journald::layer() {
        Ok(layer) => {
            registry.with(layer).init();
        },
        Err(e) => {
            registry.init();
            tracing::error!("couldn't connect to journald: {}", e);
        }
    }

    // Implement initialisation:
    /* 
        - Are there any hanging .stamp files, indicating that a scheduled run didn't happen?
            - Reschedule these based on a flag similar to "Persist" which says, "immediately_if_missed" 
            or "next_24h_trough". Default to "next_24h_trough".
        - Read config of all existing gtimers. Verify service is present. Schedule them all, reading 
        from previous runs to establish next window.
     */

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(10);
    
    let mut mask = SigSet::empty();
    mask.add(Signal::SIGHUP);
    mask.add(Signal::SIGTERM);
    mask.add(Signal::SIGINT);
    sigprocmask(SigmaskHow::SIG_BLOCK, Some(&mask), None)?;
    let sfd = SignalFd::new(&mask)?;

    poll.registry().register(
        &mut SourceFd(&sfd.as_raw_fd()),
        SIGNAL_TOKEN,
        Interest::READABLE
    )?;

    // Leave this label for easy debugging with rust-analyzer.
    '_poll_events: loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            match event.token() {
                SIGNAL_TOKEN => {
                    // Internal loop in case of queued signals (someone spamming SIGHUP?).
                    'signal: loop {
                        match sfd.read_signal() {
                            Ok(Some(sig)) => {
                                // It not valid signal, do nothing.
                                if let Ok(signal) = Signal::try_from(sig.ssi_signo as i32) {
                                    match signal {
                                        Signal::SIGHUP => tracing::info!("SIGHUP Received, rebooting cfg..."),
                                        Signal::SIGINT | Signal::SIGTERM => {
                                            tracing::warn!("Terminating signal received. Shutting down cleanly...");
                                            return Ok(())
                                        },
                                        // Shouldn't really be receiving other signals.
                                        _ => {}
                                    }
                                }
                            },
                            Ok(None) => {
                                // No more signals waiting/partial read, return to main loop.
                                break 'signal;
                            },
                            Err(e) => {
                                /* Let systemd handle restart: error beyond control of this process, 
                                so might as well just let systemd restart us to re-register the 
                                signal events. */
                                eprintln!("{e}");
                                std::process::exit(1);
                            }
                        }
                    }
                },
                // Put schedule refresh && process spawning (D-Bus/varlink) logic here.
                _ => unreachable!()
            }
        }
    }

    // let forecast = forecast::Forecast::fetch_fw_24h_postcode(
    //     "YO1", time::UtcDateTime::now()
    // ).unwrap();

    // println!("{:#?}", forecast);
}