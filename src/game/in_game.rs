use super::*;

pub struct InGame;

#[derive(Component)]
struct MyGameCamera;

#[derive(Resource)]
pub struct ImgHandle(Handle<Image>);

#[derive(Resource, Default)]
pub struct SpritesInPlay(Vec<Entity>);

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("startup");

    // + img resource
    let img_handle: Handle<Image> = asset_server.load("gfx/Overworld.png");
    commands.insert_resource(ImgHandle(img_handle));

    // add camera entity
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            ..Default::default()
        },
        MyGameCamera,
    ));
}

pub fn enter(mut commands: Commands, img: Res<ImgHandle>) {
    info!("enter");

    // + sprite
    let sprite = commands
        .spawn(SpriteBundle {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: img.0.clone(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        })
        .id();

    // + track sprite
    let mut sprites = SpritesInPlay::default();
    sprites.0.push(sprite);
    commands.insert_resource(sprites);
}

pub fn update() {
    // info!("update");
}

pub fn exit(mut commands: Commands, mut sprites: ResMut<SpritesInPlay>) {
    info!("exit");

    // - tracked sprite
    let sprites = std::mem::take(&mut sprites.0);
    commands.add(|world: &mut World| {
        for sprite in sprites {
            world.despawn(sprite);
        }
    });
    commands.remove_resource::<SpritesInPlay>();
}
