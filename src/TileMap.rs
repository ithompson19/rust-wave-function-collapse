use rand::{thread_rng, seq::SliceRandom};

pub mod Tile;

const MAP_SIZE: usize = 20;

pub struct TileMap {
    tiles: [[Tile::Tile; MAP_SIZE]; MAP_SIZE]
}

impl Default for TileMap {
    fn default() -> Self {
        TileMap { 
            tiles: [[Tile::Tile::default(); MAP_SIZE]; MAP_SIZE]
        }
    }
}

impl TileMap {
    pub fn collapse_map(&mut self) {
        while self.find_highest_entropy() > 1 {
            self.collapse();
            self.print_map();
        }
    }

    pub fn collapse(&mut self) {
        let lowest_entropy_coordinates: (usize, usize) = self.find_random_lowest_entropy_tile(self.find_lowest_entropy());
        self.tiles[lowest_entropy_coordinates.0][lowest_entropy_coordinates.1].collapse();
        self.propogate_neighbors(lowest_entropy_coordinates.0, lowest_entropy_coordinates.1);
    }

    fn find_random_lowest_entropy_tile(&self, lowest_entropy: u8) -> (usize, usize) {
        let mut available_tiles: Vec<(usize, usize)> = Vec::new();
        for row in 0..MAP_SIZE {
            for col in 0..MAP_SIZE {
                if self.tiles[row][col].entropy() == lowest_entropy {
                    available_tiles.push((row, col));
                }
            }
        }
        let mut rng = thread_rng();
        if let Some(tile) = available_tiles.choose(&mut rng) {
            return *tile;
        }
        else {
            return (0, 0);
        }
    }

    fn find_lowest_entropy(&self) -> u8 {
        let mut lowest_entropy: u8 = 255;
        for row in 0..MAP_SIZE {
            for col in 0..MAP_SIZE {
                let tile_entropy: u8 = self.tiles[row][col].entropy();
                if tile_entropy > 1 && tile_entropy < lowest_entropy {
                    lowest_entropy = tile_entropy;
                }
            }
        }
        return lowest_entropy;
    }

    fn find_highest_entropy(&self) -> u8 {
        let mut highest_entropy: u8 = 0;
        for row in 0..MAP_SIZE {
            for col in 0..MAP_SIZE {
                let tile_entropy: u8 = self.tiles[row][col].entropy();
                if tile_entropy > highest_entropy {
                    highest_entropy = tile_entropy;
                }
            }
        }
        return highest_entropy;
    }

    fn propogate_neighbors(&mut self, row: usize, col: usize) {
        if row > 0 && self.tiles[row - 1][col].entropy() > 1 && self.tiles[row - 1][col].propogate(self.tiles[row][col]) {
            self.propogate_neighbors(row - 1, col);
        }
        if row < MAP_SIZE - 1 && self.tiles[row + 1][col].entropy() > 1 && self.tiles[row + 1][col].propogate(self.tiles[row][col]) {
            self.propogate_neighbors(row + 1, col);
        }
        if col > 0 && self.tiles[row][col - 1].entropy() > 1 && self.tiles[row][col - 1].propogate(self.tiles[row][col]) {
            self.propogate_neighbors(row, col - 1);
        }
        if col < MAP_SIZE - 1 && self.tiles[row][col + 1].entropy() > 1 && self.tiles[row][col + 1].propogate(self.tiles[row][col]) {
            self.propogate_neighbors(row, col + 1);
        }
    }

    pub fn print_map(&self) {
        for row in self.tiles {
            let mut print_bin = String::new();
            let mut print_char = String::new();
            for tile in row {
                print_bin = format!("{} {}", print_bin, tile.print_state());
                print_char.push(tile.print_tile());
            }
            println!("{} | {}", print_bin, print_char);
        }
        println!("");
    }
}