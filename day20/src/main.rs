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

fn copy_tile(tile: &Tile) -> Tile {
  let mut copied = Vec::new();
  for line in &tile.image {
    copied.push(line.to_string());
  }
  Tile {
    id: tile.id,
    image: copied,
  }
}

/*
1 2 3
4 5 6
7 8 9

3 6 9
2 5 8
1 4 7
*/
fn rotate_counter_clockwise(mut tile: Tile) -> Tile {
  let mut rotated = Vec::new();
  let image_size = tile.image.len();

  for i in 0 .. image_size {
    rotated.push(String::new());
    for j in 0 .. image_size {
      rotated[i].push(tile.image[j].chars().nth(image_size-i-1).unwrap());
    }
  }

  tile.image = rotated;
  tile
}

fn flip_vertically(mut tile: Tile) -> Tile {
  tile.image = tile.image.iter().rev().map(|s| s.to_string()).collect();
  tile
}

fn flip_horizontally(mut tile: Tile) -> Tile {
  for i in 0 .. tile.image.len() {
    tile.image[i] = tile.image[i].chars().rev().collect::<String>();
  }
  tile
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

fn get_left_edge(
  tile: &Tile
) -> String {
  let mut left = String::new();
  for row in &tile.image {
    left.push(row.chars().next().unwrap());
  }
  left
}

fn get_right_edge(
  tile: &Tile
) -> String {
  let mut right = String::new();
  for row in &tile.image {
    right.push(row.chars().nth(row.len()-1).unwrap());
  }
  right
}

enum TextDirection {
  Forwards,
  Reverse
}

fn edge_match(
  a_edge: &String,
  b_edge: &String,
) -> Option<TextDirection> {
  if a_edge == b_edge {
    Some(TextDirection::Forwards)
  } else if a_edge == &b_edge.chars().rev().collect::<String>() {
    Some(TextDirection::Reverse)
  } else {
    None
  }
}

enum EdgeDirection {
  Up,
  Down,
  Left,
  Right
}

fn get_edges_with_direction(
  tile: &Tile
) -> Vec<(EdgeDirection, String)> {
  vec![
    (EdgeDirection::Up, tile.image[0].to_string()),
    (EdgeDirection::Down, tile.image[tile.image.len()-1].to_string()),
    (EdgeDirection::Right, get_right_edge(tile)),
    (EdgeDirection::Left, get_left_edge(tile)),
  ]
}

fn get_edges(
  tile: &Tile
) -> Vec<String> {
  get_edges_with_direction(tile).iter().map(|(_, edge)| edge.to_string()).collect()
}

fn do_tiles_connect(
  a: &Tile,
  b: &Tile,
) -> bool {
  let a_edges = get_edges(a);
  let b_edges = get_edges(b);
  
  for a_edge in &a_edges {
    for b_edge in &b_edges {
      if edge_match(a_edge, b_edge).is_some() {
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

fn tile_has_matching_edge(
  edge: &String,
  tile: &Tile,
) -> Option<(EdgeDirection, TextDirection)> {
  for (direction, other_edge) in get_edges_with_direction(tile) {
    if let Some(text_direction) = edge_match(edge, &other_edge) {
      return Some((direction, text_direction));
    }
  }
  None
}

/*
Left, no op
1
2
3

Up
3 2 1   // reverses

Right
    3   // reverses
    2
    1

Down
    *
    *
1 2 3  // does not reverse
       // if the match is reverse, we'll need to flip
*/
fn rotate_tile_edge_to_left(
  tile: &Tile,
  edge_direction: EdgeDirection,
  text_direction: TextDirection,
) -> Tile {
  match edge_direction {
    EdgeDirection::Up => {
      match text_direction {
        TextDirection::Forwards => flip_vertically(rotate_counter_clockwise(copy_tile(tile))),
        TextDirection::Reverse => rotate_counter_clockwise(copy_tile(tile)),
      }
    },
    EdgeDirection::Down => {
      match text_direction {
        TextDirection::Forwards => rotate_counter_clockwise(rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile)))),
        TextDirection::Reverse => flip_vertically(rotate_counter_clockwise(rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile))))),
      }
    },
    EdgeDirection::Left => {
      match text_direction {
        TextDirection::Forwards => copy_tile(tile),
        TextDirection::Reverse => flip_vertically(copy_tile(tile)),
      }
    },
    EdgeDirection::Right => {
      match text_direction {
        TextDirection::Forwards => flip_vertically(rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile)))),
        TextDirection::Reverse => rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile))),
      }
    },
  }
}

fn get_tile_right(
  tiles: &[Tile],
  tile_left: &Tile,
) -> Tile {
  let right_side = get_right_edge(tile_left);

  for other_tile in tiles {
    // the tile always matches with itself. exclude it
    if other_tile.id != tile_left.id {
      if let Some((edge_direction, text_direction)) = tile_has_matching_edge(&right_side, other_tile) {
        // found the matching edge. need to rotate and flip it so it matches
        return rotate_tile_edge_to_left(other_tile, edge_direction, text_direction);
      }
    }
  }

  // This should never happen. There should always be one and only one matching edge.
  Tile {
    id: 0,
    image: Vec::new()
  }
}

/*
Up
1 2 3

Right
    1
    2
    3

Down
    *
    *
3 2 1  // reversed

Left, // reversed
3
2
1
*/
fn rotate_tile_edge_to_top(
  tile: &Tile,
  edge_direction: EdgeDirection,
  text_direction: TextDirection,
) -> Tile {
  match edge_direction {
    EdgeDirection::Up => {
      match text_direction {
        TextDirection::Forwards => copy_tile(tile),
        TextDirection::Reverse => flip_horizontally(copy_tile(tile)),
      }
    },
    EdgeDirection::Down => {
      match text_direction {
        TextDirection::Forwards => flip_horizontally(rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile)))),
        TextDirection::Reverse => rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile))),
      }
    },
    EdgeDirection::Left => {
      match text_direction {
        TextDirection::Forwards => flip_horizontally(rotate_counter_clockwise(rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile))))),
        TextDirection::Reverse => rotate_counter_clockwise(rotate_counter_clockwise(rotate_counter_clockwise(copy_tile(tile)))),
      }
    },
    EdgeDirection::Right => {
      match text_direction {
        TextDirection::Forwards => rotate_counter_clockwise(copy_tile(tile)),
        TextDirection::Reverse => flip_horizontally(rotate_counter_clockwise(copy_tile(tile))),
      }
    },
  }
}

fn get_tile_below(
  tiles: &[Tile],
  tile_above: &Tile,
) -> Tile {
  let bottom_side = get_right_edge(tile_above);

  for other_tile in tiles {
    // the tile always matches with itself. exclude it
    if other_tile.id != tile_above.id {
      if let Some((edge_direction, text_direction)) = tile_has_matching_edge(&bottom_side, other_tile) {
        // found the matching edge. need to rotate and flip it so it matches
        return rotate_tile_edge_to_top(other_tile, edge_direction, text_direction);
      }
    }
  }

  // This should never happen. There should always be one and only one matching edge.
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

  for _i in 1..dim {
    let prev_tile = result[0].last().cloned().unwrap();
    result[0].push(get_tile_right(tiles, &prev_tile));
  }

  for i in 1..dim {
    result.push(Vec::new());
    for j in 0..dim {
      let above_tile = &(result[i-1][j].clone());
      result[i].push(get_tile_below(tiles, above_tile));
    }
  }

  result
}

fn print_tiles(assembled_tiles: &[Vec<Tile>]) {
  for row in assembled_tiles {
    for col in row {
      print!("{} ", col.id);
    }
    println!("");
  }
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
  print_tiles(&assembled_tiles);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}