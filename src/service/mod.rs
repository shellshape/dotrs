use crate::util::git::DEFAULT_COMMIT_AUTHOR;
use crate::{config::Config, util::dotfiles};
use anyhow::Result;
use debounce::EventDebouncer;
use log::{debug, error, info};
use notify::{EventKind, RecursiveMode, Watcher};
use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

#[derive(Debug)]
enum Event {
    Apply,
    Update,
    Pull,
}

pub struct Service {
    cfg: Config,
    dir: PathBuf,
    rx: Receiver<Event>,

    _watcher: Box<dyn Watcher>,
}

macro_rules! matches_event {
    ($kind:expr) => {
        matches!(
            $kind,
            EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(_)
        )
    };
}

impl Service {
    pub fn new(
        cfg: Config,
        apply_delay: Duration,
        update_delay: Duration,
        pull_frequency: Duration,
    ) -> Result<Service> {
        let (tx, rx) = mpsc::channel();

        let apply_tx = tx.clone();
        let apply_db = EventDebouncer::new(apply_delay, move |_| {
            apply_tx.send(Event::Apply).expect("channel send")
        });

        let update_tx = tx.clone();
        let update_db = EventDebouncer::new(update_delay, move |_| {
            update_tx.send(Event::Update).expect("channel send")
        });

        let dir = cfg.stage_dir.as_ref().to_owned().canonicalize()?;

        let stage_dir = dir.clone();
        let mut watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, _>| match res {
                Err(err) => error!("file watch error: {err}"),
                Ok(event) if matches_event!(event.kind) => {
                    debug!("FS event received: {event:?}");

                    if event
                        .paths
                        .iter()
                        .any(|p| p.strip_prefix(&stage_dir).unwrap().starts_with(".git"))
                    {
                        return;
                    }

                    apply_db.put(true);
                    update_db.put(true);
                }
                Ok(event) => debug!("event does not match anything: {event:?}"),
            })?;

        watcher.watch(&dir, RecursiveMode::Recursive)?;

        let pull_tx = tx.clone();
        thread::spawn(move || loop {
            pull_tx.send(Event::Pull).expect("channel send");
            thread::sleep(pull_frequency);
        });

        Ok(Service {
            cfg,
            dir,
            rx,
            _watcher: Box::new(watcher),
        })
    }

    pub fn watch(&self) -> Result<()> {
        info!("Watching {} ...", self.dir.to_string_lossy());

        for event in &self.rx {
            info!("received event: {event:?}");
            match event {
                Event::Apply => {
                    if let Err(err) = dotfiles::apply(&self.cfg, None::<&str>) {
                        error!("failed applying dotfiles: {err}");
                    }
                }
                Event::Update => {
                    if let Err(err) =
                        dotfiles::update(&self.cfg, DEFAULT_COMMIT_AUTHOR, None::<&str>)
                    {
                        error!("failed applying dotfiles: {err}");
                    }
                }
                Event::Pull => {
                    if let Err(err) = dotfiles::pull(&self.cfg) {
                        error!("failed pulling dotfiles stage: {err}");
                    } else if let Err(err) = dotfiles::apply(&self.cfg, None::<&str>) {
                        error!("failed applying dotfiles after pull: {err}");
                    }
                }
            }
        }

        Ok(())
    }
}
