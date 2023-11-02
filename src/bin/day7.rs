use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Read,
    path::PathBuf,
    str::SplitAsciiWhitespace,
};

fn main() -> anyhow::Result<()> {
    let total_disk_space = 70_000_000;
    let min_free_space = 30_000_000;

    let mut buffer = String::new();
    _ = File::open("inputs/day7_input.txt")?.read_to_string(&mut buffer)?;

    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::default();
    let mut path_register: Vec<PathBuf> = Vec::default();

    for line in buffer.lines() {
        let mut split_line = line.split_ascii_whitespace();
        match split_line.next() {
            Some("$") => handle_command(&mut path_register, &mut split_line),
            Some(entry) => handle_listing(&path_register, &mut dir_sizes, entry),
            _ => unreachable!("has to be command or ls result"),
        }
    }

    dbg!(&dir_sizes);

    let result_part1 = dir_sizes
        .iter()
        .filter(|(_, v)| v <= &&100000)
        .map(|(_, v)| v)
        .sum::<u32>();

    dbg!(result_part1);

    let used_space = dir_sizes.get(&PathBuf::from("/")).unwrap();
    let folder_to_remove_target_size = used_space - (total_disk_space - min_free_space);

    let mut delta_sizes = dir_sizes
        .iter()
        .filter_map(|(k, v)| {
            if v.ge(&folder_to_remove_target_size) {
                Some((v.clone(), k.clone()))
            } else {
                None
            }
        })
        .collect::<BTreeMap<u32, PathBuf>>();
    let result_part2 = delta_sizes.first_entry().unwrap();
    dbg!(result_part2.key());

    Ok(())
}

fn handle_command(path_register: &mut Vec<PathBuf>, iter: &mut SplitAsciiWhitespace) {
    match iter.next() {
        Some("cd") => match iter.next() {
            Some("..") => {
                path_register.pop().expect("can not cd out of /");
            }
            Some(dir) => {
                let mut last = path_register.last().cloned().unwrap_or_default();
                last.push(dir);
                path_register.push(last);
            }
            _ => panic!("missing arg for cd"),
        },
        Some(_) => {}
        _ => unreachable!(),
    }
}
fn handle_listing(path: &Vec<PathBuf>, dirs: &mut HashMap<PathBuf, u32>, list_entry: &str) {
    println!("Scanning dir {:?}", path);
    if let Ok(size) = u32::from_str_radix(list_entry, 10) {
        for dir in path {
            dirs.entry(dir.clone())
                .and_modify(|e| *e += size)
                .or_insert(size);
        }
    }
}
