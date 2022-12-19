use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,

    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

#[derive(Debug, Clone, Copy)]
struct RobotCost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct RobotBlueprint {
    material: Material,
    cost: RobotCost,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot: RobotBlueprint,
    clay_robot: RobotBlueprint,
    obsidian_robot: RobotBlueprint,
    geode_robot: RobotBlueprint,
}

impl Resources {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn can_produce(&self, blueprint: &RobotBlueprint) -> bool {
        self.ore >= blueprint.cost.ore
            && self.clay >= blueprint.cost.clay
            && self.obsidian >= blueprint.cost.obsidian
    }

    fn miner_needed(&self, material: Material, blueprints: &Blueprint) -> bool {
        if material == Material::Geode {
            return true;
        }

        [
            &blueprints.ore_robot,
            &blueprints.clay_robot,
            &blueprints.obsidian_robot,
            &blueprints.geode_robot,
        ]
        .iter()
        .any(|blueprint| match material {
            Material::Ore => blueprint.cost.ore > self.ore_robots,
            Material::Clay => blueprint.cost.clay > self.clay_robots,
            Material::Obsidian => blueprint.cost.obsidian > self.obsidian_robots,
            _ => unreachable!(),
        })
    }

    fn produce(mut self, to_produce: Option<RobotBlueprint>) -> Self {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        if let Some(RobotBlueprint {
            material,
            cost:
                RobotCost {
                    ore,
                    clay,
                    obsidian,
                },
        }) = to_produce
        {
            match material {
                Material::Ore => self.ore_robots += 1,
                Material::Clay => self.clay_robots += 1,
                Material::Obsidian => self.obsidian_robots += 1,
                Material::Geode => self.geode_robots += 1,
            }

            self.ore -= ore;
            self.clay -= clay;
            self.obsidian -= obsidian;
        }

        self
    }
}

impl TryFrom<&str> for Blueprint {
    type Error = ();

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut iter = src
            .split(&[' ', ':'])
            .filter_map(|word| word.parse::<u32>().ok());

        let id = iter.next().ok_or(())?;

        let ore_robot = RobotBlueprint {
            material: Material::Ore,
            cost: RobotCost {
                ore: iter.next().ok_or(())?,
                clay: 0,
                obsidian: 0,
            },
        };

        let clay_robot = RobotBlueprint {
            material: Material::Clay,
            cost: RobotCost {
                ore: iter.next().ok_or(())?,
                clay: 0,
                obsidian: 0,
            },
        };

        let obsidian_robot = RobotBlueprint {
            material: Material::Obsidian,
            cost: RobotCost {
                ore: iter.next().ok_or(())?,
                clay: iter.next().ok_or(())?,
                obsidian: 0,
            },
        };

        let geode_robot = RobotBlueprint {
            material: Material::Geode,
            cost: RobotCost {
                ore: iter.next().ok_or(())?,
                clay: 0,
                obsidian: iter.next().ok_or(())?,
            },
        };

        Ok(Self {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        })
    }
}

impl Blueprint {
    fn quality_level(&self) -> u32 {
        self.id * self.max_geode_production(24)
    }

    fn max_geode_production(&self, minutes: u32) -> u32 {
        let mut queue = VecDeque::new();
        let mut cache = HashMap::new();
        queue.push_back((Resources::new(), 0));

        while let Some((resources, time_passed)) = queue.pop_front() {
            let prev_best = *cache.get(&time_passed).unwrap_or(&0);

            if resources.geode < prev_best {
                continue;
            }

            cache.insert(time_passed, resources.geode);

            if time_passed >= minutes {
                continue;
            }

            if resources.can_produce(&self.geode_robot) {
                queue.push_back((resources.produce(Some(self.geode_robot)), time_passed + 1));
                continue;
            }

            queue.push_back((resources.produce(None), time_passed + 1));
            for blueprint in [&self.ore_robot, &self.clay_robot, &self.obsidian_robot] {
                if resources.can_produce(blueprint)
                    && resources.miner_needed(blueprint.material, self)
                {
                    queue.push_back((resources.produce(Some(*blueprint)), time_passed + 1));
                }
            }
        }

        *cache.get(&minutes).unwrap_or(&0)
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .filter_map(|line| Blueprint::try_from(line).ok())
        .collect()
}

fn main() {
    let blueprints = parse_input(include_str!("../input.txt"));

    let total_quality_level = blueprints
        .iter()
        .fold(0, |acc, blueprint| acc + blueprint.quality_level());

    println!(
        "Total quality level of all blueprints: {}",
        total_quality_level
    );
}
