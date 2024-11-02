#![allow(unused)]

use std::collections::{HashMap, HashSet};

mod cpm {
// Define a Task struct to represent each task.
#[derive(Debug)]
struct Task {
    name: String,
    duration: u32, // Duration in days
    dependencies: HashSet<String>, // Tasks that need to be completed before this one
}

// Define a Project struct to manage tasks.
#[derive(Debug)]
struct Project {
    tasks: HashMap<String, Task>,
}


impl Project {
    fn add_task(&mut self, name: &str, duration: u32, dependencies: HashSet<String>) {
        let task = Task {
            name: name.to_string(),
            duration,
            dependencies,
        };
        self.tasks.insert(name.to_string(), task);
    }
}

impl Project {
    fn calculate_critical_path(&self) -> (u32, Vec<String>) {
        let mut memo: HashMap<String, (u32, Vec<String>)> = HashMap::new();
        let mut longest_path = 0;
        let mut critical_path = Vec::new();

        // Recursive function to calculate the longest path for each task.
        fn dfs(task_name: &str, project: &Project, memo: &mut HashMap<String, (u32, Vec<String>)>) -> (u32, Vec<String>) {
            if let Some(&(duration, ref path)) = memo.get(task_name) {
                return (duration, path.clone());
            }

            let task = project.tasks.get(task_name).unwrap();
            let mut max_duration = task.duration;
            let mut max_path = vec![task_name.to_string()];

            for dependency in &task.dependencies {
                let (dep_duration, mut dep_path) = dfs(dependency, project, memo);
                if dep_duration + task.duration > max_duration {
                    max_duration = dep_duration + task.duration;
                    dep_path.push(task_name.to_string());
                    max_path = dep_path;
                }
            }

            memo.insert(task_name.to_string(), (max_duration, max_path.clone()));
            (max_duration, max_path)
        }

        // Calculate the critical path for each task.
        for task_name in self.tasks.keys() {
            let (duration, path) = dfs(task_name, self, &mut memo);
            if duration > longest_path {
                longest_path = duration;
                critical_path = path;
            }
        }

        (longest_path, critical_path)
    }
}

}



fn main() {
    let mut project = Project {
        tasks: HashMap::new(),
    };

    // Example: Adding tasks to the project
    project.add_task("Project Planning", 5, HashSet::new());
    project.add_task("Requirements Gathering", 3, HashSet::from(["Project Planning".to_string()]));
    project.add_task("Implementation Planning", 3, HashSet::from(["Project Planning".to_string()]));
    project.add_task("Testing & Review", 3, HashSet::from(["Requirements Gathering".to_string(), "Implementation Planning".to_string()]));
    project.add_task("Deployment & Training", 5, HashSet::from(["Requirements Gathering".to_string(), "Implementation Planning".to_string(), "Testing & Review".to_string()]));

    // Calculate the critical path
    let (duration, critical_path) = project.calculate_critical_path();

    // Output the results
    println!("Critical Path Duration: {} days", duration);
    println!("Critical Path: {:?}", critical_path);
}