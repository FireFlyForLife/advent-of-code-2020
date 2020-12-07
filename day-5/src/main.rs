use std::convert::TryFrom;
use std::collections::HashSet;

mod puzzle_input;

#[derive(Debug, PartialEq, Eq)]
enum SpacePartitionInstruction {
    Front,
    Back,
    Left,
    Right
}

impl TryFrom<char> for SpacePartitionInstruction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'F' => Ok(SpacePartitionInstruction::Front),
            'B' => Ok(SpacePartitionInstruction::Back),
            'L' => Ok(SpacePartitionInstruction::Left),
            'R' => Ok(SpacePartitionInstruction::Right),
            _ => Err(())
        }
    }
}

struct ChairPosition {
    row: i32,
    column: i32,
}

// If I was programming for real I would improve this function
fn get_chair_position(boarding_pass: &Vec<SpacePartitionInstruction>) -> ChairPosition {
    let mut row_range = (0, 127);
    let mut column_range = (0, 7);

    for instruction in boarding_pass {
        match instruction {
            SpacePartitionInstruction::Front => { 
                row_range.1 = row_range.0 + (row_range.1 - row_range.0) / 2;
            },
            SpacePartitionInstruction::Back => { 
                row_range.0 = row_range.0 + (row_range.1 - row_range.0) / 2;
            },
            SpacePartitionInstruction::Left => {
                column_range.1 = column_range.0 + (column_range.1 - column_range.0) / 2;
            },
            SpacePartitionInstruction::Right => {
                column_range.0 = column_range.0 + (column_range.1 - column_range.0) / 2;
            }
        }
    }

    ChairPosition{ row: row_range.1, column: column_range.1 }
}

fn get_chair_uid(chair: &ChairPosition) -> i32 {
    chair.row * 8 + chair.column
}

fn main() {
    let boarding_passes: Vec<Vec<SpacePartitionInstruction>> = puzzle_input::BOARDING_PASSES.lines()
        .map(|line| line.chars().map(SpacePartitionInstruction::try_from).filter_map(Result::ok).collect())
        .collect();
    
    let mut chair_uids = HashSet::with_capacity(boarding_passes.len());
    let mut largest_chair_uid = i32::min_value();
    for boarding_pass in &boarding_passes {
        let chair_position = get_chair_position(boarding_pass);
        let chair_uid = get_chair_uid(&chair_position);

        chair_uids.insert(chair_uid);
        largest_chair_uid = i32::max(largest_chair_uid, chair_uid);
    }

    let mut missing_chair_uid = None;
    for i in 1..(128*8-1) {
        if !chair_uids.contains(&i) && chair_uids.contains(&(i-1)) && chair_uids.contains(&(i+1)) {
            missing_chair_uid = Some(i);
            break;
        }
    }

    println!("Largest chair unique id: {}", largest_chair_uid);
    println!("Missing chair uid: {:?}", missing_chair_uid);
}
