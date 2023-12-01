use advent_of_code_2022::{Aoc, Input};
use std::io::BufRead;

#[derive(Debug)]
enum Resource {
    Ore,
    Clay,
    Obisidian,
}

#[derive(Debug)]
struct Cost {
    resource: Resource,
    value: usize,
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: (Cost, Cost),
    geode_robot_cost: (Cost, Cost),
}

impl From<String> for Blueprint {
    fn from(value: String) -> Self {
        let mut split = value.split(" ").skip(6);

        let ore_robot_cost = Cost {
            resource: Resource::Ore,
            value: split.next().unwrap().parse().unwrap(),
        };

        let mut split = split.skip(5);

        let clay_robot_cost = Cost {
            resource: Resource::Ore,
            value: split.next().unwrap().parse().unwrap(),
        };

        let mut split = split.skip(5);
        let ore_value = split.next().unwrap().parse().unwrap();
        let mut split = split.skip(2);
        let clay_value = split.next().unwrap().parse().unwrap();

        let obsidian_robot_cost = (
            Cost {
                resource: Resource::Ore,
                value: ore_value,
            },
            Cost {
                resource: Resource::Clay,
                value: clay_value,
            },
        );

        let mut split = split.skip(5);
        let ore_value = split.next().unwrap().parse().unwrap();
        let mut split = split.skip(2);
        let obsidian_value = split.next().unwrap().parse().unwrap();

        let geode_robot_cost = (
            Cost {
                resource: Resource::Ore,
                value: ore_value,
            },
            Cost {
                resource: Resource::Obisidian,
                value: obsidian_value,
            },
        );

        Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Collection {
    robots: usize,
    collected: usize,
}

#[derive(Default, Debug, Clone)]
struct ResourceCollection {
    ore: Collection,
    clay: Collection,
    obsidian: Collection,
    geode: Collection,
}

impl ResourceCollection {
    fn update(&mut self) {
        if self.ore.robots > 0 {
            self.ore.collected += self.ore.robots;
        }
        if self.clay.robots > 0 {
            self.clay.collected += self.clay.robots;
        }
        if self.obsidian.robots > 0 {
            self.obsidian.collected += self.obsidian.robots;
        }
        if self.geode.robots > 0 {
            self.geode.collected += self.geode.robots;
        }
    }

    // create all possible robots and return their updated resource collections
    fn robot_posibilities(&self, blueprint: &Blueprint) -> Vec<ResourceCollection> {
        let mut posibilities = vec![];

        if self.ore.collected >= blueprint.ore_robot_cost.value {
            let mut updated = self.clone();
            updated.ore.robots += 1;
            updated.ore.collected -= blueprint.ore_robot_cost.value;
            posibilities.push(updated);
        }

        if self.ore.collected >= blueprint.clay_robot_cost.value {
            let mut updated = self.clone();
            updated.clay.robots += 1;
            updated.ore.collected -= blueprint.clay_robot_cost.value;
            posibilities.push(updated);
        }

        if self.ore.collected >= blueprint.obsidian_robot_cost.0.value
            && self.clay.collected >= blueprint.obsidian_robot_cost.1.value
        {
            let mut updated = self.clone();
            updated.obsidian.robots += 1;
            updated.ore.collected -= blueprint.obsidian_robot_cost.0.value;
            updated.clay.collected -= blueprint.obsidian_robot_cost.1.value;
            posibilities.push(updated);
        }

        if self.ore.collected >= blueprint.geode_robot_cost.0.value
            && self.obsidian.collected >= blueprint.geode_robot_cost.1.value
        {
            let mut updated = self.clone();
            updated.geode.robots += 1;
            updated.ore.collected -= blueprint.geode_robot_cost.0.value;
            updated.obsidian.collected -= blueprint.geode_robot_cost.1.value;
            posibilities.push(updated);
        }

        return posibilities;
    }
}

fn recursive_brute_force(
    minutes: usize,
    blueprint: &Blueprint,
    mut resource_collection: ResourceCollection,
) -> usize {
    if minutes == 0 {
        return resource_collection.geode.collected;
    }

    resource_collection.update();

    let mut max = 0;

    if resource_collection.ore.collected > 2 {
        for rc in resource_collection
            .robot_posibilities(blueprint)
            .into_iter()
        {
            let collected = recursive_brute_force(minutes - 1, blueprint, rc);
            if collected > max {
                max = collected
            }
        }
    }

    let collected = recursive_brute_force(minutes - 1, blueprint, resource_collection);
    if collected > max {
        max = collected
    }

    return max;
}

fn optimum_geods_within_minutes(blueprint: &Blueprint, minutes: usize) -> usize {
    let mut resource_collection = ResourceCollection::default();
    resource_collection.ore.robots = 1;
    return recursive_brute_force(minutes, blueprint, resource_collection);
}

fn part1(input: Input) -> usize {
    let blueprints: Vec<Blueprint> = input.lines().map(|l| Blueprint::from(l.unwrap())).collect();
    return optimum_geods_within_minutes(&blueprints[0], 24);
}

fn part2(input: Input) -> u32 {
    todo!()
}

fn main() {
    let mut aoc = Aoc::new();
    aoc.part("one", part1);
    aoc.part("two", part2);
    aoc.run();
}
