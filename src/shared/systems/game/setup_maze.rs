// src\shared\systems\game\setup_maze.rs

use std::fmt;
use bevy::prelude::*;
use crate::shared::components::*;


#[derive(Clone)]
pub struct Cell {
    pub walkable: bool,
}

#[derive(Resource)]
pub struct Grid {
    data: Vec<u32>,
    columns: usize,
    rows: usize,
    cell_width: f32,
    cell_height: f32,
    world_offset: Vec2,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl Grid {
    pub fn from_maze(maze: &Vec<Vec<u32>>, cell_width: f32, cell_height: f32, world_offset: Vec2) -> Grid {
        let columns = maze[0].len();
        let rows = maze.len();
        let data = maze.iter().flat_map(|row| row.iter()).cloned().collect();
        
        let width = columns as f32 * cell_width;
        let height = rows as f32 * cell_height;
        let left = world_offset.x;
        let right = left + width;
        let bottom = world_offset.y;
        let top = bottom + height;

        Grid {
            data,
            columns,
            rows,
            cell_width,
            cell_height,
            world_offset,
            left,
            right,
            bottom,
            top,
        }
    }

    pub fn world_to_grid(&self, position: Vec3) -> (i32,i32) {
        let x = (((position.x - self.world_offset.x) / self.cell_width) + 0.5).floor() as i32;
        let y = (((position.y - self.world_offset.y) / self.cell_height) + 0.5).floor() as i32;

        (x, y)
        
    //     let cell = (x, y);

    // // for debugging...
    //     let gtw = self.grid_to_world((x,y));
    //     let x2 = (((gtw.x - self.world_offset.x) / self.cell_width) + 0.5).floor() as i32;
    //     let y2 = (((gtw.y - self.world_offset.y) / self.cell_height) + 0.5).floor() as i32;
    //     println!("DEBUG ---");
    //     println!("DEBUG world_to_grid {:?} : {:?} ", position , (x,y));
    //     println!("DEBUG grid_to_world {:?} : {:?} ", gtw, (x2, y2));
    //     println!("DEBUG ---");
    // //

    //     cell
    }

    pub fn grid_to_world(&self, cell: (i32, i32)) -> Vec3 {
        let (x, y) = cell;
        let world_x = (x as f32 * self.cell_width) + self.world_offset.x;
        let world_y = (y as f32 * self.cell_height) + self.world_offset.y;
        Vec3::new(world_x, world_y, 0.0)
    }


    pub fn neighbors(&self, pos: &(i32, i32)) -> Vec<((i32, i32), usize)> {
        let &(x, y) = pos;
        let directions = vec![
            (x - 1, y, 1000),
            (x + 1, y, 1000),
            (x, y - 1, 1000),
            (x, y + 1, 1000),
            (x - 1, y - 1, 1414),
            (x + 1, y - 1, 1414),
            (x - 1, y + 1, 1414),
            (x + 1, y + 1, 1414),
        ];

        let valid_neighbors = directions
            .into_iter()
            .filter(|&(nx, ny, _cost)| {
                if self.is_walkable((nx, ny)) {
                    if _cost == 1414 {
                        self.is_walkable((nx, y)) && self.is_walkable((x, ny))
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .map(|(nx, ny, cost)| ((nx, ny), cost))
            .collect();

        // println!("Neighbors of {:?}: {:?}", pos, valid_neighbors);
        valid_neighbors
    }

    pub fn is_inside_grid(&self, world_position: Vec3) -> bool {
        let x = world_position.x as f32;
        let y = world_position.y as f32;
        x >= self.left - 2.0 * self.cell_width 
        && x <= self.right + 2.0 * self.cell_width
        && y >= self.bottom - 2.0 * self.cell_height
        && y <= self.top + 2.0 * self.cell_height
    }

    pub fn is_walkable(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;

        if x >= 0 && x < self.columns as i32 && y >= 0 && y < self.rows as i32 {
            self.data[(y as usize) * self.columns + (x as usize)] == 0
        } else {
            true
        }
    }

}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Grid ({} rows, {} columns):", self.rows, self.columns)?;

        // Print column numbers across the top
        write!(f, "    ")?;
        for x in 0..self.columns {
            write!(f, "{:4}", x)?;
        }
        writeln!(f)?;

        // Print the grid with row numbers on the left
        for y in (0..self.rows).rev() {
            write!(f, "{:2}:", y)?;
            for x in 0..self.columns {
                let index = y * self.columns + x;
                write!(f, "{:4}", self.data[index])?;
            }
            writeln!(f)?;
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
    let world_offset:Vec2 = Vec2::new(-1600.0, -3000.0);

    // let mut maze = [
    let mut maze_array = vec![
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1],
        [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
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

    println!("Dungeon Grid:\n{:?}", grid);
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


