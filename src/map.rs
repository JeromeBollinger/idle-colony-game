use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub const MAP_WIDTH: u32 = 32;
pub const MAP_HEIGTH: u32 = 32;
pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGTH: u32 = 16;


pub fn initiate_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize {
        x: MAP_WIDTH,
        y: MAP_HEIGTH,
    };
    let tile_size = TilemapTileSize {
        x: TILE_WIDTH as f32,
        y: TILE_HEIGTH as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    let wall_texture_handle: Vec<Handle<Image>> = vec![
        asset_server.load("transparent.png"),
        asset_server.load("regolith2.png"),
    ];
    let floor_texture_handle: Vec<Handle<Image>> = vec![asset_server.load("regolith.png")];

    let (wall_tilemap_entity, wall_tile_storage) = create_map(
        map_size,
        &mut commands,
        MapKind::SolidMap(
            Map::from_string(&MAP1),
            Map::from_string(&MAP1),
        ),
    );
    let (floor_tilemap_entity, floor_tile_storage) = create_map(
        map_size,
        &mut commands,
        MapKind::AssetIndexMap(Map::from_string(&MAP_FLOOR)),
    );

    commands.entity(wall_tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: wall_tile_storage,
            texture: TilemapTexture::Vector(wall_texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
            ..Default::default()
        },
        WallMapComponent {},
    ));

    commands.entity(floor_tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: floor_tile_storage,
        texture: TilemapTexture::Vector(floor_texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

#[derive(Component)]
pub struct WallMapComponent {}

fn create_map(
    map_size: TilemapSize,
    commands: &mut Commands,
    map: MapKind,
) -> (Entity, TileStorage) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    match map {
        MapKind::AssetIndexMap(m) => {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = commands
                        .spawn(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(m.map()[x as usize][y as usize]),
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }
        MapKind::SolidMap(tm, sm) => {
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    if sm.map()[x as usize][y as usize] == 0 {
                        let tile_entity = commands
                            .spawn((TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                texture_index: TileTextureIndex(tm.map()[x as usize][y as usize]),
                                ..Default::default()
                            },))
                            .id();
                        tile_storage.set(&tile_pos, tile_entity);
                    } else {
                        let tile_entity = commands
                            .spawn((
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(tilemap_entity),
                                    texture_index: TileTextureIndex(
                                        tm.map()[x as usize][y as usize],
                                    ),
                                    ..Default::default()
                                },
                                Solid {},
                            ))
                            .id();
                        tile_storage.set(&tile_pos, tile_entity);
                    }
                }
            }
        }
    }

    (tilemap_entity, tile_storage)
}

#[derive(Component)]
pub struct Solid {}

struct Map(Vec<Vec<u32>>);

impl Map {
    fn from_string(map_path: &str) -> Self {
        let mut map: Vec<Vec<u32>> = vec![vec![]];

        for line in map_path.lines() {
                for (x, c) in line.chars().enumerate() {
                    match c.to_digit(10) {
                        Some(i) => map[x].push(i),
                        None => map[x].push(0),
                    }
                    map.push(vec![]);
                }
        }
        Map(map)
    }
    fn map(&self) -> &Vec<Vec<u32>> {
        &self.0
    }
}

enum MapKind {
    AssetIndexMap(Map),
    SolidMap(Map, Map),
}

const MAP1: &str = "11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111000000000000001111111111
11111111000000000000001111111111
11111111000000000000111111111111
11111111000000000000111111111111
10000000000000000000111111111111
10000000000000000000111111111111
11111111000000000000111111111111
11111111000000000000001111111111
11111111000000000000001111111111
11111111000000000000001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111100001111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111
11111111111111111111111111111111";

const MAP_FLOOR: &str = "00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000
00000000000000000000000000000000";
