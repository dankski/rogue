const MAP_HEIGHT: u32 = 25;
const MAP_WIDTH: u32 = 100;

#[derive(Clone)]
pub struct Tile {
  pub ch: i32,
  pub walkable: bool,
}

pub struct Map {
  height: u32,
  width: u32,
  pub tiles: Vec<Vec<Tile>>
}


impl Map {

  pub fn new() -> Self {
    Self {
      height: MAP_HEIGHT,
      width: MAP_WIDTH,
      tiles: Vec::new()
    }
  }

  pub fn create_map_tiles(&mut self) {
    self.tiles = (0..self.height as usize)
    .map(|_| (0..self.width as usize)
    .map(|_| Tile{ch: '#' as i32, walkable: false}).collect())
    .collect();
  }

  pub fn setup_map(&mut self) {
    for y in 5..15 as usize {
      for x in 40..60 as usize {
        self.tiles[y][x] = Tile{ch: '.' as i32, walkable: true}
      }
    }
  }
 
  pub fn height(&self) -> u32 {
      self.height
  }

  pub fn width(&self) -> u32 {
      self.width
  }

  pub fn tiles(&self) -> Vec<Vec<Tile>> {
      self.tiles.clone()
  }
}