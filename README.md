# Bevy-Tests

Dummy project to study Bevy.

#### Notes

###### Assets
- https://opengameart.org/art-search-advanced?keys=&title=&field_art_tags_tid_op=or&field_art_tags_tid=&name=&field_art_type_tid%5B%5D=9&field_art_licenses_tid%5B%5D=4&sort_by=count&sort_order=DESC&items_per_page=24&Collection=
- https://kenney.nl/assets?q=2d
- https://itch.io/c/845926/libre-game-assets

###### Other

```rust
// Parameters
//
// https://docs.rs/bevy/0.10.0/bevy/ecs/system/index.html#system-parameter-list
//
// - &World - everything
// - Entities - all entities, incl. their alive/dead status and location
// - Components - idk
// - Commands - queue changes to the world (add/rm entities/components/resources)
// - Query<>
// - [Res<>/ResMut<>] - direct global resources
//  - [NonSend<>/NonSendMut<>] - like Res/ResMut, but always on the main thread
//  - EventReader<>/EventWritter<> - like a global resource, but behind commands
//   - RemovedComponents - event reader when an entity loses a component
//  - Local<> - system-local and parameter-local resources
// - SystemName - used for debbug; - SystemChangeTick - idk
// - Bundles - set of components
//  - Archetypes - same as bundles but with no extra components

// bevy::app::CoreSet - default App system sets
// - First - time, assets
// - PreUpdate - scene, keys
// - StateTransitions
// - FixedUpdate
// - Update - default
// - PostUpdate - audio, animation, visibility, bounds, clip, transform, camera
// - Last - window

// TODO: debug lines
// - https://github.com/Toqozz/bevy_debug_lines
// TODO: asset loader
// - https://github.com/NiklasEi/bevy_asset_loader
// TODO: handle input
// - https://github.com/leafwing-studios/leafwing-input-manager
// TODO: use tilemap
// - https://github.com/StarArawn/bevy_ecs_tilemap
// TODO: particle system
// - https://github.com/djeedai/bevy_hanabi
```
