mod TileMap;

fn main() {
    let mut map: TileMap::TileMap = TileMap::TileMap::default();
    map.collapse_map();
}
