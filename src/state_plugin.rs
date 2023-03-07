use std::marker::PhantomData;
use std::sync::Mutex;

use bevy::ecs::schedule::{common_conditions, SystemConfig};
use bevy::prelude::*;

/// Creates a builder for a state-related system flow Plugin.
pub fn on_variant<Marker, State>(state_variant: State) -> OnVariant<Marker, State> {
    OnVariant::new(state_variant)
}

/// Builder for a state-related system flow Plugin.
pub struct OnVariant<Marker, State> {
    state_variant: State,
    _marker: PhantomData<Marker>,
}

impl<Marker, State> OnVariant<Marker, State> {
    /// Creates the builder.
    pub fn new(state_variant: State) -> Self {
        Self {
            state_variant,
            _marker: PhantomData,
        }
    }

    /// Runs `startup` on the first time that the state "enters" the variant.
    ///
    /// Runs before the `enter` (which runs *each* time the state "enters" the variant).  
    /// Runs only once, and also only once, flushes commands before `enter`.
    pub fn with_startup<M>(self, startup: impl IntoSystemConfig<M>) -> WithStartup<Marker, State> {
        WithStartup::new(self.state_variant, startup.into_config())
    }

    /// Indicates there's nothing to setup for the first time in particular that the state "enters" the variant.
    pub fn skip_startup(self) -> WithStartup<Marker, State> {
        WithStartup {
            state_variant: self.state_variant,
            startups: Mutex::new(vec![]),
            _marker: self._marker,
        }
    }
}

pub struct WithStartup<Marker, State> {
    state_variant: State,
    startups: Mutex<Vec<SystemConfig>>,
    _marker: PhantomData<Marker>,
}

impl<Marker, State> WithStartup<Marker, State> {
    pub fn new(state_variant: State, startup: SystemConfig) -> Self {
        Self {
            startups: Mutex::new(vec![startup]),
            state_variant,
            _marker: PhantomData,
        }
    }

    /// Runs another `startup` on the first time that the state "enters" the variant.
    ///
    /// Runs before the `enter` (which runs *each* time the state "enters" the variant).  
    /// Runs only once, and also only once, flushes commands before `enter`.
    pub fn with_startup<M>(mut self, startup: impl IntoSystemConfig<M>) -> Self {
        self.startups.get_mut().unwrap().push(startup.into_config());
        self
    }

    /// Runs each time the state "enters" the variant.
    /// Flushes commands after running.
    // TODO: check if the schedule already flushes
    pub fn with_enter<M>(self, enter: impl IntoSystemConfig<M>) -> WithEnter<Marker, State> {
        WithEnter::new(
            self.state_variant,
            Mutex::into_inner(self.startups).unwrap(),
            enter.into_config(),
        )
    }

    /// Indicates there's nothing to setup for each time the state "enters" the variant.
    pub fn skip_enter(self) -> WithEnter<Marker, State> {
        WithEnter {
            state_variant: self.state_variant,
            startups: self.startups,
            enters: Mutex::new(vec![]),
            _marker: self._marker,
        }
    }
}

pub struct WithEnter<Marker, State> {
    state_variant: State,
    startups: Mutex<Vec<SystemConfig>>,
    enters: Mutex<Vec<SystemConfig>>,
    _marker: PhantomData<Marker>,
}

impl<Marker, State> WithEnter<Marker, State> {
    pub fn new(
        state_variant: State,
        startup: impl IntoIterator<Item = SystemConfig>,
        enter: SystemConfig,
    ) -> Self {
        Self {
            startups: Mutex::new(startup.into_iter().collect()),
            enters: Mutex::new(vec![enter]),
            state_variant,
            _marker: PhantomData,
        }
    }

    /// Runs each time the state "enters" the variant.
    /// Flushes commands after running.
    // TODO: check if the schedule already flushes
    pub fn with_enter<M>(mut self, enter: impl IntoSystemConfig<M>) -> Self {
        self.enters.get_mut().unwrap().push(enter.into_config());
        self
    }

    /// Runs each time the state is "active" in the variant.
    pub fn with_update<M>(self, update: impl IntoSystemConfig<M>) -> WithUpdate<Marker, State> {
        WithUpdate::new(
            self.state_variant,
            Mutex::into_inner(self.startups).unwrap(),
            Mutex::into_inner(self.enters).unwrap(),
            update.into_config(),
        )
    }

    /// Indicates there's nothing to run when the state is "active" in the variant.
    pub fn skip_update(self) -> WithUpdate<Marker, State> {
        WithUpdate {
            state_variant: self.state_variant,
            startups: self.startups,
            enters: self.enters,
            updates: Mutex::new(vec![]),
            _marker: self._marker,
        }
    }
}

pub struct WithUpdate<Marker, State> {
    state_variant: State,
    startups: Mutex<Vec<SystemConfig>>,
    enters: Mutex<Vec<SystemConfig>>,
    updates: Mutex<Vec<SystemConfig>>,
    _marker: PhantomData<Marker>,
}

impl<Marker, State> WithUpdate<Marker, State> {
    pub fn new(
        state_variant: State,
        startup: impl IntoIterator<Item = SystemConfig>,
        enter: impl IntoIterator<Item = SystemConfig>,
        update: SystemConfig,
    ) -> Self {
        Self {
            startups: Mutex::new(startup.into_iter().collect()),
            enters: Mutex::new(enter.into_iter().collect()),
            updates: Mutex::new(vec![update]),
            state_variant,
            _marker: PhantomData,
        }
    }

    /// Runs each time the state is "active" in the variant.
    pub fn with_update<M>(mut self, update: impl IntoSystemConfig<M>) -> Self {
        self.updates.get_mut().unwrap().push(update.into_config());
        self
    }

    /// Runs each time the state "leaves" the variant.
    pub fn with_exit<M>(self, exit: impl IntoSystemConfig<M>) -> WithExit<Marker, State> {
        WithExit::new(
            self.state_variant,
            Mutex::into_inner(self.startups).unwrap(),
            Mutex::into_inner(self.enters).unwrap(),
            Mutex::into_inner(self.updates).unwrap(),
            exit.into_config(),
        )
    }

    /// Indicates there's nothing to cleanup each time state "leaves" the variant.
    pub fn skip_exit(self) -> WithExit<Marker, State> {
        WithExit {
            state_variant: self.state_variant,
            startups: self.startups,
            enters: self.enters,
            updates: self.updates,
            exits: Mutex::new(vec![]),
            _marker: self._marker,
        }
    }
}

pub struct WithExit<Marker, State> {
    state_variant: State,
    startups: Mutex<Vec<SystemConfig>>,
    enters: Mutex<Vec<SystemConfig>>,
    updates: Mutex<Vec<SystemConfig>>,
    exits: Mutex<Vec<SystemConfig>>,
    _marker: PhantomData<Marker>,
}

impl<Marker, State> WithExit<Marker, State> {
    pub fn new(
        state_variant: State,
        startup: impl IntoIterator<Item = SystemConfig>,
        enter: impl IntoIterator<Item = SystemConfig>,
        update: impl IntoIterator<Item = SystemConfig>,
        exit: SystemConfig,
    ) -> Self {
        Self {
            startups: Mutex::new(startup.into_iter().collect()),
            enters: Mutex::new(enter.into_iter().collect()),
            updates: Mutex::new(update.into_iter().collect()),
            exits: Mutex::new(vec![exit]),
            state_variant,
            _marker: PhantomData,
        }
    }

    /// Runs each time the state "leaves" the variant.
    pub fn with_exit<M>(mut self, exit: impl IntoSystemConfig<M>) -> Self {
        self.exits.get_mut().unwrap().push(exit.into_config());
        self
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StateSets<S> {
    StartupEnter(PhantomData<S>),
    StartupEnterFlush(PhantomData<S>),
    Enter(PhantomData<S>),
    EnterFlush(PhantomData<S>),
}

impl<Marker, State> Plugin for WithExit<Marker, State>
where
    Marker: Send + Sync + 'static,
    State: bevy::prelude::States,
{
    fn build(&self, app: &mut App) {
        let state_variant = self.state_variant.clone();
        let startups: Vec<_> = std::mem::take(Mutex::lock(&self.startups).unwrap().as_mut());
        let enters: Vec<_> = std::mem::take(Mutex::lock(&self.enters).unwrap().as_mut());
        let updates: Vec<_> = std::mem::take(Mutex::lock(&self.updates).unwrap().as_mut());
        let exits: Vec<_> = std::mem::take(Mutex::lock(&self.exits).unwrap().as_mut());

        let startup_set = &StateSets::<State>::StartupEnter(PhantomData);
        let startup_flush_set = &StateSets::<State>::StartupEnterFlush(PhantomData);

        let enter_set = &StateSets::<State>::Enter(PhantomData);
        let enter_flush_set = &StateSets::<State>::EnterFlush(PhantomData);

        let on_enter_schedule = app
            .get_schedule_mut(OnEnter(state_variant.clone()))
            .unwrap();

        match (!startups.is_empty(), !enters.is_empty()) {
            // no Startup nor Enter systems
            (false, false) => {}

            // only Startup systems
            (true, false) => {
                // "startup" set -> "startup-flush" set
                let flush_after_startup = startup_flush_set.clone().after(startup_set.clone());
                // once("startup" set -> "startup-flush" set)
                let startup_once = flush_after_startup.run_if(common_conditions::run_once());
                on_enter_schedule.configure_set(startup_once);

                // "startup-flush" set += flush system
                let startup_flush = apply_system_buffers.in_set(startup_flush_set.clone());
                on_enter_schedule.add_system(startup_flush);
            }

            // only Enter systems
            (false, true) => {
                // "enter" set -> "enter-flush" set
                let flush_after_enter = enter_flush_set.clone().after(enter_set.clone());
                on_enter_schedule.configure_set(flush_after_enter);

                // "enter-flush" set += flush system
                let enter_flush = apply_system_buffers.in_set(enter_flush_set.clone());
                on_enter_schedule.add_system(enter_flush);
            }

            // both Startup and Enter Systems
            (true, true) => {
                // "startup" set -> "startup-flush" set
                let flush_after_startup = startup_flush_set.clone().after(startup_set.clone());
                // once("startup" set -> "startup-flush" set)
                let startup_once = flush_after_startup.run_if(common_conditions::run_once());
                // once("startup" set -> "startup-flush" set) -> "enter" set
                let startup_before_enter = startup_once.before(enter_set.clone());

                // "enter" set -> "enter-flush" set
                let flush_after_enter = enter_flush_set.clone().after(enter_set.clone());

                on_enter_schedule.configure_set(startup_before_enter);
                on_enter_schedule.configure_set(flush_after_enter);

                // "startup-flush" set += flush system
                let startup_flush = apply_system_buffers.in_set(startup_flush_set.clone());
                // "enter-flush" set += flush system
                let enter_flush = apply_system_buffers.in_set(enter_flush_set.clone());
                on_enter_schedule.add_system(startup_flush);
                on_enter_schedule.add_system(enter_flush);
            }
        }

        for startup in startups {
            on_enter_schedule.add_system(startup.in_set(startup_set.clone()));
        }

        for enter in enters {
            on_enter_schedule.add_system(enter.in_set(enter_set.clone()));
        }

        for update in updates {
            let update = update.in_set(OnUpdate(state_variant.clone()));
            app.add_system(update);
        }

        let on_exit_schedule = app.get_schedule_mut(OnExit(state_variant)).unwrap();
        for exit in exits {
            on_exit_schedule.add_system(exit);
        }
    }
}
