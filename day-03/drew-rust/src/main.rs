/*
Basic rust bin with runtime error and arg parsing
*/
use std::{ collections::BTreeMap, env, error::Error, fmt::{ self, Display }, hash::Hash, iter, path::Path };

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
struct SupplyItem {
    id: char,
    count: usize,
}

impl Hash for SupplyItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for SupplyItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SupplyItem {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Display for SupplyItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} ({}: {})", self.get_priority(), self.id, self.count))
    }
}

impl SupplyItem {
    fn new(id: char) -> Self {
        SupplyItem { id, count: 1 }
    }

    fn increment(&mut self) -> &mut Self {
        self.count += 1;
        self
    }

    fn get_priority(self) -> u8 {
        determine_priority(&self.id)
    }
}

fn determine_priority(id: &char) -> u8 {
    match id {
        // a = 97, z = 122 => 1..=26
        'a'..='z' => (*id as u8) - 96,
        // A = 65, Z = 90 => 27..=52
        'A'..='Z' => (*id as u8) - 38,
        _ => 0u8,
    }
}

#[derive(Debug, Clone)]
struct RucksackCompartment {
    raw_contents: String,
    contents: BTreeMap<u8, SupplyItem>,
}

impl RucksackCompartment {
    fn new(compartment_contents: &str) -> Self {
        let mut compartments: BTreeMap<u8, SupplyItem> = BTreeMap::new();

        for c in compartment_contents.chars() {
            let supply_item = SupplyItem::new(c);

            match compartments.get_mut(&determine_priority(&c)) {
                Some(existing_item) => {
                    existing_item.increment();
                }
                None => {
                    let key = determine_priority(&c);
                    compartments.insert(key, supply_item);
                }
            }
        }

        RucksackCompartment {
            raw_contents: String::from(compartment_contents),
            contents: compartments,
        }
    }

    fn common_items_with(&self, other: &Self) -> Self {
        let common_items: Vec<SupplyItem> = self.contents
            .keys()
            .filter(|id| other.contents.contains_key(*id))
            .map(|id| SupplyItem {
                id: self.contents.get(id).unwrap().id,
                count: self.contents.get(id).unwrap().count + other.contents.get(id).unwrap().count,
            })
            .collect();

        Self::new(
            &common_items
                .iter()
                .map(|i| iter::repeat(i.id).take(i.count).collect::<String>())
                .reduce(|this, other| format!("{}{}", this, other))
                .unwrap()
        )
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    size: usize,
    compartment_1: RucksackCompartment,
    compartment_2: RucksackCompartment,
}

impl Rucksack {
    fn new(rucksack_contents: &str) -> Self {
        Rucksack {
            size: rucksack_contents.len(),
            compartment_1: RucksackCompartment::new(&rucksack_contents[0..rucksack_contents.len() / 2]),
            compartment_2: RucksackCompartment::new(&rucksack_contents[rucksack_contents.len() / 2..]),
        }
    }

    fn unpack(&self) -> RucksackCompartment {
        let merged_raw_contents = format!("{}{}", self.compartment_1.raw_contents, self.compartment_2.raw_contents);
        RucksackCompartment::new(&merged_raw_contents)
    }

    fn get_common_items(&self) -> Result<Vec<SupplyItem>> {
        let common_items: Vec<SupplyItem> = self.compartment_1.contents
            .keys()
            .filter(|id| self.compartment_2.contents.contains_key(*id))
            .map(|id| SupplyItem {
                id: self.compartment_1.contents.get(id).unwrap().id,
                count: self.compartment_1.contents.get(id).unwrap().count +
                self.compartment_2.contents.get(id).unwrap().count,
            })
            .collect();

        Ok(common_items)
    }
}

#[derive(Debug, Clone)]
struct ElfGroup {
    rucksacks: Vec<Rucksack>,
}

impl ElfGroup {
    fn new() -> Self {
        ElfGroup {
            rucksacks: Vec::new(),
        }
    }

    fn add(&mut self, rucksack: Rucksack) {
        self.rucksacks.push(rucksack);
    }

    fn get_common_items(&self) -> Result<Vec<SupplyItem>> {
        let unpacked_rucksacks: Vec<RucksackCompartment> = self.rucksacks
            .iter()
            .map(|r| r.unpack())
            .collect();

        let mut unpacked_common_items: RucksackCompartment = unpacked_rucksacks
            .get(0)
            .unwrap()
            .common_items_with(unpacked_rucksacks.get(1).unwrap());
        for unpacked_rucksack in unpacked_rucksacks[2..].iter() {
            unpacked_common_items = unpacked_common_items.common_items_with(unpacked_rucksack);
        }

        Ok(
            unpacked_common_items.contents
                .iter()
                .map(|entry| entry.1.to_owned())
                .collect()
        )
    }

    fn size(&self) -> usize {
        self.rucksacks.len()
    }
}

fn parse_elf_groups(input_file_path: &str, group_size: usize) -> Result<Vec<ElfGroup>> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }
    let mut elf_groups: Vec<ElfGroup> = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        let mut current_elf_group = ElfGroup::new();
        for line in lines {
            if current_elf_group.size() >= group_size {
                elf_groups.push(current_elf_group);
                current_elf_group = ElfGroup::new();
            }
            match line {
                Ok(line) => current_elf_group.add(Rucksack::new(&line)),
                Err(err) => {
                    return Err(Box::new(err));
                }
            }
        }
        // Capture the last group
        elf_groups.push(current_elf_group);
    }

    Ok(elf_groups)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from("Must provide input file path"))));
    }
    let input_path = &args[1];
    let elf_groups = parse_elf_groups(input_path, 3)?;

    // println!("{:#?}", elf_groups);

    let mut overlap_points: u32 = 0;
    for elf_group in elf_groups.iter() {
        for rucksack in elf_group.rucksacks.iter() {
            for supply_item in rucksack.get_common_items()?.iter() {
                overlap_points += supply_item.get_priority() as u32;
                // println!("{}", supply_item)
            }
        }
    }

    println!("Total Points: {}", overlap_points);

    let mut elf_group_points: u32 = 0;
    for elf_group in elf_groups.iter() {
        let common_items = elf_group.get_common_items()?;
        for supply_item in common_items {
            elf_group_points += supply_item.get_priority() as u32;
            // println!("{}", supply_item);
        }
    }

    println!("Elf Groups Total Points: {}", elf_group_points);

    Ok(())
}