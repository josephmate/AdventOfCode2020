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

fn get_bottom_edge(
  tile: &Tile
) -> String {
  tile.image.iter().last().unwrap().to_string()
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    (EdgeDirection::Down, get_bottom_edge(tile)),
    (EdgeDirection::Right, get_right_edge(tile)),
    (EdgeDirection::Left, get_left_edge(tile)),
  ]
}

fn get_edges(
  tile: &Tile
) -> Vec<String> {
  get_edges_with_direction(tile).iter().map(|(_, edge)| edge.to_string()).collect()
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

fn do_tiles_connect(
  a: &Tile,
  b: &Tile,
) -> Option<(EdgeDirection, TextDirection)> {
  let a_edges = get_edges_with_direction(a);
  let b_edges = get_edges(b);

  for (edge_direction, a_edge) in &a_edges {
    for b_edge in &b_edges {
      if let Some(text_direction) = edge_match(a_edge, b_edge) {
        return Some((*edge_direction, text_direction));
      }
    }
  }

  None
}

fn get_connected_edge(
  tile: &Tile,
  tiles: &[Tile],
) -> Vec<(EdgeDirection, TextDirection)> {
  tiles.iter()
    .filter(|other_tile| tile.id != other_tile.id)
    .filter(|other_tile| tile.id != other_tile.id)
    .map(|other_tile| do_tiles_connect(tile, other_tile))
    .filter(|option| option.is_some())
    .map(|option| option.unwrap())
    .collect()
}

fn get_corners(
  tiles: &[Tile],
) -> Vec<&Tile> {
  let mut corner_tiles: Vec<&Tile> = Vec::new();

  for tile in tiles {
    if get_connected_edge(tile, tiles).len() == 2 {
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
    image: vec![
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
    ],
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
  let bottom_side = get_bottom_edge(tile_above);

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
    image: vec![
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
      "..........".to_string(),
    ],
  }
}

fn rotate_until_top_right_corner(
  tile: Tile,
  tiles: &[Tile],
) -> Tile {
  let connected_edges: Vec<EdgeDirection> = get_connected_edge(&tile, tiles)
    .iter()
    .map(|(edge_direction, _)| *edge_direction)
    .collect();

  if connected_edges.contains(&EdgeDirection::Right) && connected_edges.contains(&EdgeDirection::Down) {
    /*
        1
        2
    1 2 3
    */
    tile
  } else if connected_edges.contains(&EdgeDirection::Left) && connected_edges.contains(&EdgeDirection::Down) {
    /*
    1
    2
    3 2 1
    */
    rotate_counter_clockwise(tile)
  } else if connected_edges.contains(&EdgeDirection::Left) && connected_edges.contains(&EdgeDirection::Down) {
    /*
    1 2 3
    2
    3
    */
    rotate_counter_clockwise(rotate_counter_clockwise(tile))
  } else {
    /*
    1 2 3
        2
        1
    */
    rotate_counter_clockwise(rotate_counter_clockwise(rotate_counter_clockwise(tile)))
  }
}

fn assemble_puzzle<'a>(
  tiles: &'a [Tile],
  corner_tiles: &'a [&Tile]
) -> Vec<Vec<Tile>> {
  let dim = get_dimension(tiles);
  let mut result = Vec::new();
  result.push(Vec::new());
  let first_corner = rotate_until_top_right_corner(
    copy_tile(corner_tiles.iter().next().unwrap()),
    tiles
  );
  // need to clone since some tiles will need to be flipped or rotated
  result[0].push(first_corner);

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

fn get_image(assembled_tiles: &[Vec<Tile>]) -> Vec<String> {
  let mut result = Vec::new();
  
  let tile_rows = assembled_tiles.len();
  let rows_in_tile = assembled_tiles[0][0].image.len();
  let tile_cols = assembled_tiles.len();
  let cols_in_tile = assembled_tiles[0][0].image.len();
  for tile_row in 0..tile_rows {
    for row_in_tile in 1..(rows_in_tile-1) {
      let mut row = String::new();
      for tile_col in 0 .. tile_cols {
        for col_in_tile in 1..(cols_in_tile-1) {
          row.push(assembled_tiles[tile_row][tile_col].image[row_in_tile].chars().nth(col_in_tile).unwrap());
        }
      }
      result.push(row);
    }
  }

  result
}

fn search_for_pattern(
  image: &[String]
) -> usize {
  let pattern = vec![
    "                  # ".to_string(),
    "#    ##    ##    ###".to_string(),
    " #  #  #  #  #  #   ".to_string(),
  ];

  for i in 0..image.len()-2 {
    for j in 0..image.len()-19 {
      let mut wave_count = 0;
      let mut is_monster = true;
      for i_delta in 0..3 {
        for j_delta in 0..20 {
          if image[i+i_delta][j+j_delta]
          // TODO try all rotations and flips of image
        }
      }
    }
  }

  0
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
  let assembled_image = get_image(&assembled_tiles);
  println!("{}", &assembled_image.join("\n"));
  println!("{}", search_for_pattern(&assembled_image));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_samples() {
      
    }

}