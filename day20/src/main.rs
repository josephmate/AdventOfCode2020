// .lines()
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Tile {
  id: usize,
  image: Vec<String>
}

fn parse_input(
  lines: &mut dyn Iterator<Item=String>
) -> Vec<Tile> {
  let mut result = Vec::new();
  let mut current_line_opt = lines.next();
  while current_line_opt.is_some() {
    let tile_line = current_line_opt.unwrap();
    let mut id_tokens = tile_line.split(' ');
    id_tokens.next(); // ignore tile
    let almost_id = id_tokens.next().unwrap();
    let mut almost_id_tokens = almost_id.split(':');
    let id = almost_id_tokens.next().unwrap().parse::<usize>().unwrap();
    let mut image = Vec::new();

    while let Some(current_line) = lines.next() {
      if current_line.is_empty() {
        break;
      }
      image.push(current_line);
    }
    result.push(Tile {
      id,
      image,
    });
    current_line_opt = lines.next();
  }
  result
}

fn get_edges(
  tile: &Tile
) -> Vec<String> {
  let mut left = String::new();
  let mut right = String::new();
  for row in &tile.image {
    left.push(row.chars().next().unwrap());
    right.push(row.chars().nth(row.len()-1).unwrap());
  }

  vec![
    tile.image[0].to_string(),
    tile.image[tile.image.len()-1].to_string(),
    left,
    right
  ]
}

fn do_tiles_connect(
  a: &Tile,
  b: &Tile,
) -> bool {
  let a_edges = get_edges(a);
  let b_edges = get_edges(b);
  
  for a_edge in &a_edges {
    for b_edge in &b_edges {
      if a_edge == b_edge || a_edge == &b_edge.chars().rev().collect::<String>() {
      //if a_edge == b_edge || a_edge.chars().eq(b_edge.chars().rev()) {
        return true;
      }
    }
  }

  false
}

fn get_corners(
  tiles: &[Tile],
) -> Vec<&Tile> {
  let mut corner_tiles: Vec<&Tile> = Vec::new();

  for tile in tiles {
    if tiles.iter()
        .filter(|other_tile| tile.id != other_tile.id)
        .filter(|other_tile| do_tiles_connect(tile, other_tile))
        .count() == 2
    {
      corner_tiles.push(tile);
    }
  }

  corner_tiles
}

fn sqrt(a: usize) -> usize {
  for i in 1..a {
    if i * i == a {
      return i;
    }
  }
  a
}

fn get_dimension(
  tiles: &[Tile],
) -> usize {
  sqrt(tiles.len())
}

fn get_tile_right(
  tiles: &[Tile],
  tile_left: &Tile,
) -> Tile {
  Tile {
    id: 0,
    image: Vec::new()
  }
}

fn get_tile_below(
  tiles: &[Tile],
  tile_above: &Tile,
) -> Tile {
  Tile {
    id: 0,
    image: Vec::new()
  }
}

fn assemble_puzzle<'a>(
  tiles: &'a [Tile],
  corner_tiles: &'a [&Tile]
) -> Vec<Vec<Tile>> {
  let dim = get_dimension(tiles);
  let mut result = Vec::new();
  result.push(Vec::new());
  let first_corner = corner_tiles.iter().next().unwrap();
  // need to clone since some tiles will need to be flipped or rotated
  result[0].push(Tile {
    id: first_corner.id,
    image: first_corner.image.to_vec(),
  });

  for i in 1..dim {
    let prev_tile = result[0].last().cloned().unwrap();
    result[0].push(get_tile_right(tiles, &prev_tile));
  }

  for i in 1..dim {
    for j in 0..dim {
      if j == 0 {
        let prev_tile = result[i-1].first().cloned().unwrap();
        result.push(Vec::new());
        result[i].push(get_tile_below(tiles, &prev_tile));
      } else {
        let prev_tile = result[i].last().cloned().unwrap();
        result[i].push(get_tile_right(tiles, &prev_tile));
      }
    }
  }

  return result;
}

fn main() {
  // let args: Vec<String> = env::args().collect();
  // &mut std::io::stdin().lock().lines()
  let tiles = parse_input(&mut std::io::stdin().lock().lines()
    .map(|line_result| line_result.unwrap())
  );

  let corner_tiles = get_corners(&tiles);
  println!("{}", corner_tiles.iter().map(|tile| tile.id).product::<usize>());

  let assembled_tiles = assemble_puzzle(&tiles, &corner_tiles);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}