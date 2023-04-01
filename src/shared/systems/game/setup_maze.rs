// src\shared\systems\game\setup_maze.rs

//use std::cell::Cell;

use std::fmt;

use bevy::prelude::*;
use crate::shared::components::*;
// use pathfinding::prelude::astar;

#[derive(Clone)]
pub struct Cell {
    pub walkable: bool,
}

#[derive(Resource)]
pub struct Grid {
    data: Vec<Vec<u32>>,
    cell_width: f32,
    cell_height: f32,
    world_offset: Vec2,
}
impl Grid {
    pub fn from_maze(maze: &Vec<Vec<u32>>, cell_width: f32, cell_height: f32, world_offset: Vec2) -> Grid {
        Grid {
            data: maze.clone(),
            cell_width,
            cell_height,
            world_offset,
        }
    }

    pub fn columns(&self) -> usize {
        self.data[0].len()
    }
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> f32 {
        self.columns() as f32 * self.cell_width
    }
    pub fn height(&self) -> f32 {
        self.rows() as f32 * self.cell_height
    }

    pub fn left(&self) -> f32 {
        self.world_offset.x
    }
    pub fn right(&self) -> f32 {
        self.left() + self.width()
    }
    pub fn bottom(&self) -> f32 {
        self.world_offset.y
    }
    pub fn top(&self) -> f32 {
        self.bottom() + self.height()
    }



    // pub fn world_to_grid(&self, position: Vec3) -> (usize, usize) {
    //     let x = (position.x / self.cell_width).round() as usize;
    //     let y = (position.y / self.cell_height).round() as usize;
    //     (x, y)
    // }
    pub fn world_to_grid(&self, position: Vec3) -> (i32,i32) {
        let x = ((position.x - self.world_offset.x) / self.cell_width).floor() as i32;
        let y = ((position.y - self.world_offset.y) / self.cell_height).floor() as i32;
        (x, y)
    }

    // pub fn grid_to_world(&self, grid_position: Vec2) -> Vec3 {
    //     Vec3::new(
    //         grid_position.x * self.cell_width as f32 + self.cell_width as f32 / 2.0,
    //         grid_position.y * self.cell_height as f32 + self.cell_height as f32 / 2.0,
    //         0.0,
    //     )
    // }
    // pub fn grid_to_world(&self, position: (usize, usize)) -> Vec3 {
    //     let x = position.0 as f32 * self.cell_width + self.cell_width / 2.0;
    //     let y = position.1 as f32 * self.cell_height + self.cell_height / 2.0;

    //     Vec3::new(x, y, 0.0)
    // }

    // pub fn grid_to_world(&self, grid_position: (usize, usize)) -> Vec3 {
    //     let x = (grid_position.0 as f32 * self.cell_width) - 1600.0;
    //     let y = (grid_position.1 as f32 * self.cell_height) - 1000.0;
    //     Vec3::new(x, y, 0.0) ? and offset someting?
    // }

    pub fn neighbors(&self, pos: &(i32, i32)) -> Vec<((i32, i32), usize)> {
        let &(x, y) = pos;
        let directions = vec![
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
        ];

        let valid_neighbors = directions
            .into_iter()
            .filter(|&(nx, ny)| self.is_walkable((nx, ny)))
            .map(|pos| (pos, 1))
            .collect();

        println!("Neighbors of {:?}: {:?}", pos, valid_neighbors);
        valid_neighbors
    }


    // pub fn neighbors(&self, pos: &(i32, i32)) -> Vec<((i32, i32), usize)> {
    //     let &(x, y) = pos;
    //     let directions = vec![
    //         (x.saturating_sub(1), y),
    //         (x + 1, y),
    //         (x, y.saturating_sub(1)),
    //         (x, y + 1),
    //     ];

    //     let valid_neighbors = directions
    //         .into_iter()
    //         .filter(|&(nx, ny)| self.is_walkable((nx, ny)))
    //         .map(|pos| (pos, 1))
    //         .collect();

    //     println!("Neighbors of {:?}: {:?}", pos, valid_neighbors);
    //     valid_neighbors
    // }


    // pub fn neighbors(&self, pos: &(usize, usize)) -> Vec<((usize, usize), usize)> {
    //     let &(x, y) = pos;
    //     let directions = vec![
    //         (x.saturating_sub(1), y),
    //         (x + 1, y),
    //         (x, y.saturating_sub(1)),
    //         (x, y + 1),
    //     ];

    //     directions
    //         .into_iter()
    //         .filter(|&(nx, ny)| self.is_walkable((nx, ny)))
    //         .map(|pos| (pos, 1))
    //         .collect()
    // }


    // pub fn neighbors(&self, position: &(usize, usize)) -> Vec<(usize, usize)> {
    //     let &(x, y) = position;
    //     let possible_neighbors = [
    //         (x as isize - 1, y as isize),
    //         (x as isize + 1, y as isize),
    //         (x as isize, y as isize - 1),
    //         (x as isize, y as isize + 1),
    //     ];

    //     possible_neighbors
    //         .iter()
    //         .filter_map(|&(x, y)| {
    //             if x >= 0 && x < self.data[0].len() as isize && y >= 0 && y < self.data.len() as isize {
    //                 let position = (x as usize, y as usize);
    //                 if self.is_walkable(position) {
    //                     Some(position)
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect()
    // }


    // pub fn neighbors(&self, position: &Vec2) -> Vec<Vec2> {
    //     let mut neighbors = Vec::new();
    //     let offsets = [
    //         Vec2::new(-1.0, 0.0),
    //         Vec2::new(1.0, 0.0),
    //         Vec2::new(0.0, -1.0),
    //         Vec2::new(0.0, 1.0),
    //     ];

    //     for offset in &offsets {
    //         let neighbor = *position + *offset;
    //         if self.is_inside_grid(neighbor) && self.is_walkable(neighbor) {
    //             neighbors.push(neighbor);
    //         }
    //     }

    //     neighbors
    // }

    // fn is_inside_grid(&self, position: Vec2) -> bool {
    //     position.x >= 0.0 && position.x < self.width as f32 && position.y >= 0.0 && position.y < self.height as f32
    // }
    pub fn is_inside_grid(&self, world_position: Vec3) -> bool {
        let x = world_position.x as f32;
        let y = world_position.y as f32;
        x >= self.left() - 3.0 * self.cell_width 
        && x <= self.right() + 3.0 * self.cell_width
        && y >= self.bottom() - 3.0 * self.cell_height
        && y <= self.top() + 3.0 * self.cell_height
    }

    // pub fn is_walkable(&self, position: (usize, usize)) -> bool {
    //     let (x, y) = position;
    //     self.data[y][x] == 0
    // }
    pub fn is_walkable(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;
        // let width = self.data[0].len();
        // let height = self.data.len();

        if x >= 0 && x < self.columns() as i32 && y >= 0 && y < self.rows() as i32 {
            self.data[y as usize][x as usize] == 0
        } else {
            true
        }
    }


    // fn is_walkable(&self, position: Vec2) -> bool {
    //     let x = position.x as usize;
    //     let y = position.y as usize;

    //     self.data[y][x] == 0
    // }

    // fn is_walkable(&self, position: Vec2) -> bool {
    //     let x = position.x as usize;
    //     let y = position.y as usize;

    //     self.grid[y][x] == 0
    // }

    // pub fn from_maze(maze: &Vec<Vec<u32>>) -> Grid {
    //     let height = maze.len();
    //     let width = maze[0].len();
    //     let mut cells = vec![vec![Cell { walkable: false }; width]; height];

    //     for (i, row) in maze.iter().enumerate() {
    //         for (j, cell) in row.iter().enumerate() {
    //             cells[i][j].walkable = *cell == 0;
    //         }
    //     }

    //     Grid { cells, width, height }
    // }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter().rev() {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}


pub fn setup_maze(
    mut commands: Commands,
    resource_cache: Res<ResourceCache>,
) {
    println!("Setup Maze");
    //let wall_images = &resource_cache.wall_images;

    // distance from world center 0,0 to bottom left of maze.
    let world_offset:Vec2 = Vec2::new(-1600.0, -1000.0);

    // let mut maze = [
    let mut maze_array = vec![
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1],
    ];
    maze_array.reverse();

    let wall_image = resource_cache.get_collidable_image(CollidableClassifier::Wall(WallShape::Straight));
    let wall_width = wall_image.width_i32();
    let wall_height = wall_image.height_i32();

    // Convert the maze array into a Grid
    let maze: Vec<Vec<u32>> = maze_array.iter().map(|row| row.to_vec()).collect();
    let grid = Grid::from_maze(&maze, wall_width as f32, wall_height as f32, world_offset);

    // let maze: Vec<Vec<u32>> = maze_array.iter().map(|row| row.to_vec()).collect();
    // let grid = Grid::from_maze(&maze);

    println!("Grid:\n{:?}", grid);
    // Add the Grid as a Bevy resource
    commands.insert_resource(grid);

    // Spawn Wall blocks into the game.
    for (i, row) in maze.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 1 {
                let x = (j * wall_width as usize) as f32 + world_offset.x;
                let y = (i * wall_height as usize) as f32 + world_offset.y;
                commands.spawn(WallBundle {
                    game_piece: GamePiece,
                    sprite_bundle: SpriteBundle {
                        texture: wall_image.handle(),
                        transform: Transform::from_xyz(x, y, -1.0),
                        ..default()
                    },
                    wall: Wall { 
                        shape: WallShape::Straight,
                        image: wall_image.clone(),
                    },
                });
            }
        }
    }
}


