use std::marker::PhantomData;
use std::sync::Mutex;

use bevy::ecs::schedule::{common_conditions, SystemConfig};
use bevy::prelude::*;

/// Creates a builder for a state-related system flow Plugin.
pub fn on_variant<Marker, State>(state_variant: State) -> Builder<Marker, State> {
    Builder::empty(state_variant)
}

/// Builder for a state-related system flow Plugin.
///
/// Schedule: Set flow
/// OnEnter(state-variant): Startup(once) -> Startup-flush(once) -> Enter -> EnterFlush
/// Update(state-variant): default flow
/// OnExit(state-variant): default flow
// TODO: should there be an optional 'Shutdown' schedule/set? (which would reverse and clear the Startup?)
//
// Note: `Mutex`es are used because:
// - The `Plugin`'s `build` fn uses `&self` and requires it to be `Sync`;
// - The `App`'s `add_system` methods takes the systems by value;
// - Those systems don't implement `Clone`.
pub struct Builder<Marker, State> {
    state_variant: State,
    startup_systems: Mutex<Vec<SystemConfig>>,
    enter_systems: Mutex<Vec<SystemConfig>>,
    update_systems: Mutex<Vec<SystemConfig>>,
    exit_systems: Mutex<Vec<SystemConfig>>,
    _marker: PhantomData<Marker>,
}

impl<Marker, State> Builder<Marker, State> {
    pub fn empty(state_variant: State) -> Self {
        Self {
            startup_systems: Mutex::new(vec![]),
            enter_systems: Mutex::new(vec![]),
            update_systems: Mutex::new(vec![]),
            exit_systems: Mutex::new(vec![]),
            state_variant,
            _marker: PhantomData,
        }
    }

    /// Runs an additional `startup` system on the first time that the state "enters" the variant.
    ///
    /// Runs before the `enter` (which runs *each* time the state "enters" the variant).  
    /// Runs only once, and also only once, flushes commands before the `enter` systems.
    pub fn with_startup<M>(mut self, startup: impl IntoSystemConfig<M>) -> Self {
        self.startup_systems
            .get_mut()
            .unwrap()
            .push(startup.into_config());
        self
    }

    /// Runs an additional `enter` system each time the state "enters" the variant.
    /// Flushes commands after running `update` systems.
    // TODO: check if the schedule already flushes by default
    pub fn with_enter<M>(mut self, enter: impl IntoSystemConfig<M>) -> Self {
        self.enter_systems
            .get_mut()
            .unwrap()
            .push(enter.into_config());
        self
    }

    /// Runs an additional `update` system each time the state is "active" in the variant.
    pub fn with_update<M>(mut self, update: impl IntoSystemConfig<M>) -> Self {
        self.update_systems
            .get_mut()
            .unwrap()
            .push(update.into_config());
        self
    }

    /// Runs an additional `exit` system each time the state "leaves" the variant.
    pub fn with_exit<M>(mut self, exit: impl IntoSystemConfig<M>) -> Self {
        self.exit_systems
            .get_mut()
            .unwrap()
            .push(exit.into_config());
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

impl<Marker, State> Plugin for Builder<Marker, State>
where
    Marker: Send + Sync + 'static,
    State: bevy::prelude::States,
{
    fn build(&self, app: &mut App) {
        let state_variant = self.state_variant.clone();
        let startups: Vec<_> = std::mem::take(Mutex::lock(&self.startup_systems).unwrap().as_mut());
        let enters: Vec<_> = std::mem::take(Mutex::lock(&self.enter_systems).unwrap().as_mut());
        let updates: Vec<_> = std::mem::take(Mutex::lock(&self.update_systems).unwrap().as_mut());
        let exits: Vec<_> = std::mem::take(Mutex::lock(&self.exit_systems).unwrap().as_mut());

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
                // once("startup" set)
                let once_startup = startup_set.clone().run_if(common_conditions::run_once());
                // once("startup-flush" set)
                let once_startup_flush = startup_flush_set
                    .clone()
                    .run_if(common_conditions::run_once());

                on_enter_schedule.configure_set(once_startup);
                on_enter_schedule.configure_set(once_startup_flush);

                // "startup" set -> "startup-flush" set
                let flush_after_startup = startup_flush_set.clone().after(startup_set.clone());

                on_enter_schedule.configure_set(flush_after_startup);

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
                // once("startup" set)
                let once_startup = startup_set.clone().run_if(common_conditions::run_once());
                // once("startup-flush" set)
                let once_startup_flush = startup_flush_set
                    .clone()
                    .run_if(common_conditions::run_once());

                on_enter_schedule.configure_set(once_startup);
                on_enter_schedule.configure_set(once_startup_flush);

                // "startup" set -> "startup-flush" set
                let flush_after_startup = startup_flush_set.clone().after(startup_set.clone());
                // ("startup" set -> "startup-flush" set) -> "enter" set
                let startup_before_enter = flush_after_startup.before(enter_set.clone());

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
