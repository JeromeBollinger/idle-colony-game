use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use std::fs::File;
use std::io::*;
use std::path::Path;

pub fn initiate_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_size = TilemapSize { x: 32, y: 32 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
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
        read_map(Path::new("assets/maps/map1.txt")),
    );
    let (floor_tilemap_entity, floor_tile_storage) = create_map(
        map_size,
        &mut commands,
        read_map(Path::new("assets/maps/map_floor.txt")),
    );

    commands.entity(wall_tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: wall_tile_storage,
        texture: TilemapTexture::Vector(wall_texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
        ..Default::default()
    });

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

fn create_map(
    map_size: TilemapSize,
    commands: &mut Commands,
    index_map: Vec<Vec<u32>>,
) -> (Entity, TileStorage) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(index_map[x as usize][y as usize]),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    (tilemap_entity, tile_storage)
}

fn read_map(map_path: &Path) -> Vec<Vec<u32>> {
    let input = File::open(map_path).expect("No map found");
    let mut map: Vec<Vec<u32>> = vec![vec![]];
#[derive(Component)]
struct Solid{}

struct Map(Vec<Vec<u32>>);

impl Map {

    for (_, line) in BufReader::new(input).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.chars().enumerate() {
                match c.to_digit(10) {
                    Some(i) => map[x].push(i),
                    None => map[x].push(0),
                }
                map.push(vec![]);
            }
        }
    }
    fn map(&self) -> &Vec<Vec<u32>>{
        &self.0
    }
    map
}

enum MapKind {
    AssetIndexMap(Map),
    SolidMap(Map, Map),
}
