use prelude::*;

pub struct Day07;

type DirectoryName = String;

#[derive(Debug, Clone)]
enum FileSystemEntity {
    Directory(DirectoryName),
    File(u64),
}

#[derive(Debug)]
enum Command {
    Cd(DirectoryName),
    Ls(Vec<FileSystemEntity>),
}

#[derive(Debug, Default)]
pub struct DirectoryEntry {
    parent_dir: Option<DirectoryName>,
    children: Vec<FileSystemEntity>,
}

type FileSystem = HashMap<DirectoryName, DirectoryEntry>;

fn parse_commands(i: &str) -> Vec<Command> {
    i.split("$ ")
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| {
            if l.starts_with("cd") {
                let (_, new_directory) = l.split_once(' ').unwrap();
                Command::Cd(new_directory.to_owned())
            } else {
                let (_, output) = l.split_once('\n').unwrap();
                let entities: Vec<FileSystemEntity> = output
                    .split('\n')
                    .map(|l| {
                        let (first, second) = l.split_once(' ').unwrap();
                        if first == "dir" {
                            FileSystemEntity::Directory(second.to_owned())
                        } else {
                            FileSystemEntity::File(first.parse::<u64>().unwrap())
                        }
                    })
                    .collect();
                Command::Ls(entities)
            }
        })
        .collect()
}

fn make_child_directory_name(path: DirectoryName, directory: &DirectoryName) -> DirectoryName {
    if path.is_empty() {
        directory.clone()
    } else {
        path + directory + "/"
    }
}

fn build_filesystem(commands: &[Command]) -> FileSystem {
    let mut filesystem: FileSystem = FileSystem::new();
    filesystem.insert("/".to_owned(), DirectoryEntry::default());
    let mut current_directory: DirectoryName = String::new();

    for command in commands {
        match command {
            Command::Cd(dir) => {
                if dir == ".." {
                    current_directory = filesystem
                        .get(&current_directory)
                        .unwrap()
                        .parent_dir
                        .clone()
                        .unwrap();
                } else {
                    current_directory = make_child_directory_name(current_directory, dir);
                }
            }
            Command::Ls(entities) => {
                for entity in entities {
                    match entity {
                        FileSystemEntity::Directory(dir) => {
                            let child_directory =
                                make_child_directory_name(current_directory.clone(), dir);
                            filesystem
                                .get_mut(&current_directory)
                                .unwrap()
                                .children
                                .push(FileSystemEntity::Directory(child_directory.clone()));
                            filesystem.insert(
                                child_directory,
                                DirectoryEntry {
                                    parent_dir: Some(current_directory.clone()),
                                    children: vec![],
                                },
                            );
                        }
                        f @ FileSystemEntity::File(_) => {
                            filesystem
                                .get_mut(&current_directory)
                                .unwrap()
                                .children
                                .push(f.clone());
                        }
                    }
                }
            }
        }
    }

    filesystem
}

fn directory_size(directory: &str, filesystem: &FileSystem) -> u64 {
    filesystem
        .get(directory)
        .unwrap()
        .children
        .iter()
        .map(|c| match c {
            FileSystemEntity::Directory(dir) => directory_size(dir, filesystem),
            FileSystemEntity::File(size) => *size,
        })
        .sum::<u64>()
}

impl AocSolution for Day07 {
    type Input = HashMap<DirectoryName, u64>;
    fn process_input(input: &str) -> Self::Input {
        let commands = parse_commands(input);
        let filesystem = build_filesystem(&commands);

        filesystem
            .keys()
            .map(|d| (d.clone(), directory_size(d, &filesystem)))
            .collect()
    }

    const PART1_SOLUTION: Solution = solution(1517599);
    fn part1(input: &Self::Input) -> impl ToSolution {
        input
            .iter()
            .map(|(_, size)| size)
            .filter(|size| **size <= 100000)
            .sum::<u64>()
    }

    const PART2_SOLUTION: Solution = solution(2481982);
    fn part2(input: &Self::Input) -> impl ToSolution {
        let total_filesystem_space = 70_000_000;
        let required_unused_space = 30_000_000;
        let current_used_space = input.get("/").unwrap();
        let current_unused_space = total_filesystem_space - current_used_space;
        let need_to_delete = required_unused_space - current_unused_space;

        *input
            .iter()
            .map(|(_, size)| size)
            .filter(|size| **size >= need_to_delete)
            .min()
            .unwrap()
    }
}
