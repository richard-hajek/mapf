use crate::deps::sparse::SparseMatrix2D;
use crate::deps::state_definition::{StateEnvironment, StateStatus};
use crate::mapf::action::MAPFAction::{Commit, Move};
use crate::mapf::action::{MAPFAction, MOVES};
use crate::mapf::definition::{MAPFDefinition, ParseGridError};
use crate::mapf::state::MAPFState;
use std::{
    error::Error,
    fmt::{self, Formatter},
    fs,
    path::Path,
    sync::Arc,
};

pub struct MAPFEnvironment {
    pub definition: Arc<MAPFDefinition>,
}

impl StateEnvironment<MAPFState, MAPFAction> for MAPFEnvironment {
    fn get_initial_state(&self) -> MAPFState {
        MAPFState {
            definition: self.definition.clone(),
            units_begin: self.definition.starting_positions.clone(),
            units_available: self.definition.starting_positions.clone(),
            units_moved: SparseMatrix2D::new(self.definition.shape.0, self.definition.shape.1),
            playing: 1,
        }
    }

    fn get_actions(&self, state: &MAPFState) -> Arc<Vec<MAPFAction>> {
        let mut actions = Vec::new();

        for (a0_idx, vec_) in state.units_available.data.iter().enumerate() {
            if let Some(vec) = vec_ {
                for (a1_idx, unit) in vec.iter().enumerate() {
                    if *unit != state.playing {
                        continue
                    }

                    for (da0, da1) in MOVES {
                        let na0_ = (da0) + (a0_idx as isize);
                        let na1_ = (da1) + (a1_idx as isize);

                        if na0_ < 0 || na0_ >= self.definition.shape.0 as isize {
                            continue;
                        }

                        if na1_ < 0 || na1_ >= self.definition.shape.1 as isize {
                            continue;
                        }

                        let na0 = na0_ as usize;
                        let na1 = na1_ as usize;

                        let obstacle = self.definition.obstacles.get(na0, na1);

                        if obstacle.unwrap_or(0) != 0 {
                            continue;
                        }

                        actions.push(Move((a0_idx, a1_idx), (na0, na1)));
                    }
                }
            }
        }

        actions.push(Commit);

        Arc::new(actions)
    }

    fn next(&self, s: &MAPFState, a: &MAPFAction) -> MAPFState {
        match a {
            Commit => {
                let mut next_starting_pos = s.units_available.xor(&s.units_moved);
                next_starting_pos.bit_reduce_inline();
                MAPFState {
                    definition: s.definition.clone(),
                    units_begin: next_starting_pos.clone(),
                    units_available: next_starting_pos,
                    units_moved: SparseMatrix2D::new_by_shape(self.definition.shape),
                    playing: if s.playing == 1 { 2 } else { 1 },
                }
            }
            Move(stage1, stage2) => {
                let mut stage1_positions = s.units_available.clone();
                stage1_positions.xor_inline_by_idx(stage1.0, stage1.1, s.playing);

                let mut stage2_positions = s.units_moved.clone();
                stage2_positions.xor_inline_by_idx(stage2.0, stage2.1, s.playing);

                MAPFState {
                    definition: self.definition.clone(),
                    units_begin: s.units_begin.clone(),
                    units_available: stage1_positions,
                    units_moved: stage2_positions,
                    playing: s.playing,
                }
            }
        }
    }

    fn get_status(&self, s: &MAPFState) -> StateStatus {
        if s.units_moved.get_nnz_sum() != 0 {
            // If we're in the middle of a turn, the game is still running
            return StateStatus::Running;
        }

        let mut alive: [u64; 3] = [0, 0, 0];
        let mut goals_achieved: [u64; 3] = [0, 0, 0];

        for (a0_index, row_opt) in s.units_begin.data.iter().enumerate() {
            if let Some(row) = row_opt {
                for (a1_index, &value) in row.iter().enumerate() {
                    alive[value as usize] += 1;

                    if s.definition.goals.get(a0_index, a1_index).unwrap_or(0) != 0 {
                        goals_achieved[value as usize] += 1;
                    }
                }
            }
        }

        // Check for winners
        for i in 1..3 {  // Start from 1 to skip the 0 index (no player)
            if goals_achieved[i] == s.definition.goals_num[i] && s.definition.goals_num[i] > 0 {
                return StateStatus::Winner(i as u64);
            }
        }

        if alive[1] == 0 {
            return StateStatus::Winner(2);
        }

        if alive[2] == 0 {
            return StateStatus::Winner(1);
        }

        let player1_can_win = goals_achieved[1] < s.definition.goals_num[1] && alive[1] > 0;
        let player2_can_win = goals_achieved[2] < s.definition.goals_num[2] && alive[2] > 0;

        if !player1_can_win && !player2_can_win {
            return StateStatus::Draw;
        }

        StateStatus::Running
    }
}

impl fmt::Display for MAPFEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let MAPFDefinition {
            shape: (height, width),
            starting_positions,
            obstacles,
            goals,
            goals_num: _goals_num,
        } = &*self.definition;

        for y in 0..*height {
            for x in 0..*width {
                let mut ch = '.';

                if let Some(Some(row)) = obstacles.data.get(y) {
                    if row.get(x).copied().unwrap_or(0) == 1 {
                        ch = '#';
                    }
                }

                if ch == '.' {
                    if let Some(Some(row)) = starting_positions.data.get(y) {
                        if let Some(&val) = row.get(x) {
                            if val != 0 {
                                ch = std::char::from_digit(val as u32, 10).unwrap_or('?');
                            }
                        }
                    }
                }

                if ch == '.' {
                    if let Some(Some(row)) = goals.data.get(y) {
                        if let Some(&val) = row.get(x) {
                            if val != 0 {
                                ch = (val - 1 + b'A') as char;
                            }
                        }
                    }
                }

                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl MAPFEnvironment {
    pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let mut obstacles_data: Vec<Option<Vec<u8>>> = Vec::new();
        let mut starting_data: Vec<Option<Vec<u8>>> = Vec::new();
        let mut goals_data: Vec<Option<Vec<u8>>> = Vec::new();
        let mut goals_by_player: [u64; 3] = [0, 0, 0];

        let lines: Vec<&str> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        if lines.is_empty() {
            return Err(Box::new(ParseGridError("Empty grid".to_string())));
        }

        let a1_length = lines[0].len();
        let a0_length = lines.len();

        for (y, line) in lines.iter().enumerate() {
            if line.len() != a1_length {
                return Err(Box::new(ParseGridError(format!(
                    "Inconsistent row width at line {}",
                    y
                ))));
            }

            let mut obstacle_row = vec![0u8; a1_length];
            let mut starting_row = vec![0u8; a1_length];
            let mut goals_row = vec![0u8; a1_length];

            let mut has_obstacle = false;
            let mut has_starting = false;
            let mut has_goals = false;

            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => {}
                    '#' => {
                        obstacle_row[x] = 1;
                        has_obstacle = true;
                    }
                    '0'..='9' => {
                        starting_row[x] = ch.to_digit(10).unwrap() as u8;
                        has_starting = true;
                    }
                    'A'..='Z' => {
                        goals_row[x] = (ch as u8 - b'A') + 1;
                        has_goals = true;
                    }
                    _ => {
                        return Err(Box::new(ParseGridError(format!(
                            "Invalid character '{}' at ({},{})",
                            ch, x, y
                        ))));
                    }
                }
            }

            obstacles_data.push(if has_obstacle {
                Some(obstacle_row)
            } else {
                None
            });
            starting_data.push(if has_starting {
                Some(starting_row)
            } else {
                None
            });
            goals_data.push(if has_goals { Some(goals_row) } else { None });
        }

        for vec_ in goals_data.iter() {
            if let Some(vec) = vec_ {
                for goal_player in vec {
                    goals_by_player[*goal_player as usize] += 1;
                }
            }
        }

        Ok(Self {
            definition: Arc::new(MAPFDefinition {
                shape: (a0_length, a1_length),
                starting_positions: SparseMatrix2D {
                    data: starting_data,
                    shape: (a0_length, a1_length),
                },
                obstacles: SparseMatrix2D {
                    data: obstacles_data,
                    shape: (a0_length, a1_length),
                },
                goals: SparseMatrix2D {
                    data: goals_data,
                    shape: (a0_length, a1_length),
                },
                goals_num: goals_by_player,
            }),
        })
    }
}

#[cfg(test)]
mod tests {

    impl MAPFEnvironment {
        pub fn new() -> Self {
            Self {
                definition: Arc::new(MAPFDefinition {
                    shape: (10, 10),
                    starting_positions: SparseMatrix2D {
                        data: vec![
                            None,                                     //0
                            None,                                     //1
                            None,                                     //2
                            Some(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2]), //3
                            Some(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2]), //4
                            Some(vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 2]), //5
                            None,                                     //6
                            None,                                     //7
                            None,                                     //8
                            None,                                     //9
                        ],
                        shape: (10, 10),
                    },
                    obstacles: {
                        SparseMatrix2D {
                            data: vec![
                                None,                                     //0
                                None,                                     //1
                                None,                                     //2
                                Some(vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0]), //3
                                Some(vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0]), //4
                                Some(vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0]), //5
                                None,                                     //6
                                None,                                     //7
                                None,                                     //8
                                None,                                     //9
                            ],
                            shape: (10, 10),
                        }
                    },
                    goals: {
                        SparseMatrix2D {
                            data: vec![
                                None,                                     //0
                                None,                                     //1
                                None,                                     //2
                                Some(vec![0, 0, 2, 0, 0, 0, 0, 0, 1, 0]), //3
                                Some(vec![0, 0, 2, 0, 0, 0, 0, 0, 1, 0]), //4
                                Some(vec![0, 0, 2, 0, 0, 0, 0, 0, 1, 0]), //5
                                None,                                     //6
                                None,                                     //7
                                None,                                     //8
                                None,                                     //9
                            ],
                            shape: (10, 10),
                        }
                    },
                    goals_num: [0, 3, 3],
                }),
            }
        }
    }

    use crate::deps::sparse::SparseMatrix2D;
    use crate::deps::state_definition::StateEnvironment;
    use crate::mapf::action::MAPFAction;
    use crate::mapf::definition::MAPFDefinition;
    use crate::mapf::environment::MAPFEnvironment;
    use std::sync::Arc;

    #[test]
    fn test_init() {
        let problem = MAPFEnvironment::new();
        problem.get_initial_state();
    }

    #[test]
    fn test_move() {
        let problem = MAPFEnvironment::new();
        let initial = problem.get_initial_state();

        let action = MAPFAction::Move((3, 0), (3, 1));

        let next = problem.next(&initial, &action);

        assert_eq!(next.units_available.get(3, 0).unwrap(), 0);
        assert_eq!(next.units_available.get(3, 1).unwrap(), 0);

        assert_eq!(next.units_moved.get(3, 0).unwrap(), 0);
        assert_eq!(next.units_moved.get(3, 1).unwrap(), 1);

        println!("{}", next.to_string());

        let commited = problem.next(&next, &MAPFAction::Commit);
        problem.get_status(&commited);
    }

    #[test]
    fn test_load_maps() {
        use std::fs;
        use std::path::Path;

        let dir = Path::new("./maps/");
        for entry in fs::read_dir(dir).expect("Failed to read ./maps/") {
            let path = entry.expect("Failed to read dir entry").path();
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let result = MAPFEnvironment::new_from_file(&path);
                assert!(
                    result.is_ok(),
                    "Failed to load map {:?}: {:?}",
                    path,
                    result.err()
                );

                println!("Read {} ok", path.to_string_lossy());
                let rendered = format!("{}", result.unwrap());
                println!("See: \n{}", rendered);
            }
        }
    }

    #[test]
    fn test_map_round_trip() {
        use std::fs;
        use std::path::Path;

        let dir = Path::new("./maps/");
        for entry in fs::read_dir(dir).expect("Failed to read ./maps/") {
            let path = entry.expect("Failed to read dir entry").path();
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let original = fs::read_to_string(&path).expect("Failed to read file");
                let env = MAPFEnvironment::new_from_file(&path).expect("Failed to parse map");
                let rendered = format!("{}", env);

                let normalize = |s: &str| {
                    s.lines()
                        .map(str::trim_end)
                        .filter(|l| !l.is_empty())
                        .collect::<Vec<_>>()
                        .join("\n")
                };

                assert_eq!(
                    normalize(&original),
                    normalize(&rendered),
                    "Mismatch on map {:?}",
                    path
                );
            }
        }
    }
}
