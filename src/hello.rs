use bevy::prelude::*;

pub struct HelloPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component, Default)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Bundle)]
struct PersonBundle {
    pub this: Person,
    pub name: Name,
    pub inner: InnerComponent,
}

#[derive(Component, Default)]
struct InnerComponent {
    #[allow(dead_code)]
    pub inner: Person,
}

impl PersonBundle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            this: Person,
            inner: InnerComponent::default(),
            name: Name(name.into()),
        }
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_ppl)
            // .add_system(hello_world_system)
            .add_system(greet_ppl);
    }
}

fn add_ppl(mut cmds: Commands) {
    cmds.spawn(PersonBundle::new("x"));
    cmds.spawn(PersonBundle::new("y"));
    cmds.spawn(PersonBundle::new("z"));
    cmds.spawn(Name("silvio".into()));
}

#[allow(dead_code)]
fn hello_world_system() {
    println!("ma oeee");
}

fn greet_ppl(time: Res<Time>, mut timer: ResMut<GreetTimer>, names: Query<&Name, With<Person>>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        for name in names.iter() {
            println!("hello {}", name.0);
        }
    }
}
