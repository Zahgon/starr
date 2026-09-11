//! Reads a Custom Script event out of the environment and prints what it found.
//!
//! Run it the way a Starr app would:
//!
//! ```sh
//! sonarr_eventtype=Grab sonarr_series_title='Star Trek' \
//!   cargo run --example starrcmd_demo
//! ```

use starr_rust::starrcmd::{CmdEvent, CmdResult, Dispatcher, Event};

fn main() {
    let Ok(cmd) = CmdEvent::new() else {
        eprintln!("no Starr event found in the environment");
        std::process::exit(1);
    };

    println!("app={} event={}", cmd.app, cmd.event_type);

    let mut dispatcher = Dispatcher::new();

    dispatcher.on_unknown = Some(Box::new(|cmd| {
        println!("  (no handler registered for {} {})", cmd.app, cmd.event_type);
        Ok(())
    }));

    dispatcher.on_sonarr_grab(|grab| {
        println!("  series      = {}", grab.title);
        println!("  tvdb_id     = {}", grab.tvdb_id);
        println!("  release     = {}", grab.release_title);
        println!("  size        = {}", grab.size);
        println!("  episodes    = {:?}", grab.episode_numbers);
        println!("  titles      = {:?}", grab.episode_titles);
        println!("  air_dates   = {:?}", grab.episode_air_dates_utc);
        Ok(())
    });

    dispatcher.on_radarr_download(|download| {
        println!("  movie       = {}", download.title);
        println!("  file        = {}", download.file_path);
        println!("  is_upgrade  = {}", download.is_upgrade);
        Ok(())
    });

    if let Err(err) = dispatcher.dispatch(&cmd) {
        eprintln!("dispatch failed: {err}");
        std::process::exit(1);
    }

    // Getters refuse to decode the wrong event, exactly like the Go library.
    if cmd.event_type != Event::TEST {
        let wrong: CmdResult<_> = cmd.get_sonarr_test();
        println!("wrong-event guard: {}", wrong.unwrap_err());
    }
}
