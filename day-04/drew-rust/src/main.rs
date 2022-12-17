use std::fmt::{ self, Display };
use std::path::Path;
use std::{ env, error::Error };

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct SectionAssignment {
    range_start: u32,
    range_end: u32,
}

impl SectionAssignment {
    fn parse(input: &str) -> Option<Self> {
        match input.split_once('-') {
            Some(parts) => {
                let range_start = parts.0.parse::<u32>().unwrap();
                let range_end = parts.1.parse::<u32>().unwrap();
                Some(SectionAssignment {
                    range_start,
                    range_end,
                })
            }
            None => None,
        }
    }
}

impl Display for SectionAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.range_start, self.range_end))
    }
}

impl PartialEq for SectionAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.range_start == other.range_start && self.range_end == other.range_end
    }
}

enum AssignmentOverlap {
    NoOverlap,
    LeftContainsRight,
    Overlap,
    RightContainsLeft,
    CompleteOverlap,
}

const NO_OVERLAP: &str = "<  L > < R  >";
const LEFT_CONTAINS_RIGHT: &str = "<  L   < R >>";
const OVERLAP: &str = "<  L < > R  >";
const RIGHT_CONTAINS_LEFT: &str = "<< L >   R  >";
const COMPLETE_OVERLAP: &str = "<  L     R  >";

impl AssignmentOverlap {
    /// This method assumes that the ordering of the start values is from least to greatest for the left and right assignment inputs
    fn determine_overlap(
        left_assignment: &SectionAssignment,
        right_assignment: &SectionAssignment
    ) -> AssignmentOverlap {
        if left_assignment == right_assignment {
            Self::CompleteOverlap
        } else if
            left_assignment.range_start <= right_assignment.range_start &&
            left_assignment.range_end >= right_assignment.range_end
        {
            Self::LeftContainsRight
        } else if
            right_assignment.range_start <= left_assignment.range_start &&
            right_assignment.range_end >= left_assignment.range_end
        {
            Self::RightContainsLeft
        } else if left_assignment.range_end >= right_assignment.range_start {
            Self::Overlap
        } else {
            Self::NoOverlap
        }
    }

    fn glyph(&self) -> &'static str {
        match self {
            AssignmentOverlap::NoOverlap => NO_OVERLAP,
            AssignmentOverlap::LeftContainsRight => LEFT_CONTAINS_RIGHT,
            AssignmentOverlap::Overlap => OVERLAP,
            AssignmentOverlap::RightContainsLeft => RIGHT_CONTAINS_LEFT,
            AssignmentOverlap::CompleteOverlap => COMPLETE_OVERLAP,
        }
    }
}

impl Display for AssignmentOverlap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.glyph()))
    }
}

struct AssignmentPair {
    left_assignment: SectionAssignment,
    right_assignment: SectionAssignment,
    assignment_overlap: AssignmentOverlap,
}

impl AssignmentPair {
    fn parse(input: &str) -> Option<Self> {
        match input.split_once(',') {
            Some(parts) => {
                let left_assignment = SectionAssignment::parse(parts.0).unwrap();
                let right_assignment = SectionAssignment::parse(parts.1).unwrap();

                let assignment_overlap = if left_assignment.range_start <= right_assignment.range_start {
                    AssignmentOverlap::determine_overlap(&left_assignment, &right_assignment)
                } else {
                    AssignmentOverlap::determine_overlap(&right_assignment, &left_assignment)
                };

                Some(AssignmentPair {
                    left_assignment,
                    right_assignment,
                    assignment_overlap,
                })
            }
            None => None,
        }
    }

    fn has_overlap(&self) -> bool {
        !matches!(self.assignment_overlap, AssignmentOverlap::NoOverlap)
    }

    fn has_fully_contains(&self) -> bool {
        matches!(
            self.assignment_overlap,
            AssignmentOverlap::LeftContainsRight |
                AssignmentOverlap::RightContainsLeft |
                AssignmentOverlap::CompleteOverlap
        )
    }
}

impl Display for AssignmentPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{},{}  {}", self.left_assignment, self.right_assignment, self.assignment_overlap))
    }
}

fn parse_assignments(input_file_path: &str) -> Result<Vec<AssignmentPair>> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }
    let mut assignment_pairs: Vec<AssignmentPair> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            match line {
                Ok(line) => {
                    let assignment_pair = AssignmentPair::parse(&line);
                    match assignment_pair {
                        Some(assignment_pair) => {
                            assignment_pairs.push(assignment_pair);
                        }
                        None => (),
                    }
                }
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }
    }

    Ok(assignment_pairs)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from("Must provide input file path"))));
    }
    let input_path = &args[1];
    let assignments = parse_assignments(input_path).unwrap();

    for assignment in assignments.iter() {
        println!("{}", assignment);
    }

    let overlapping_assignment_count = assignments
        .iter()
        .filter(|assignment| assignment.has_overlap())
        .count();

    println!("Overlapping assignments: {}", overlapping_assignment_count);

    let fully_contains_assignment_count = assignments
        .iter()
        .filter(|assignment| assignment.has_fully_contains())
        .count();

    println!("Fully contains assignments: {}", fully_contains_assignment_count);

    Ok(())
}