use std::collections::HashMap;

struct Pipeline {
    name: String,
    steps: Vec<String>,
}

impl Pipeline {
    fn new(name: &str) -> Self {
        Pipeline {
            name: name.to_string(),
            steps: Vec::new(),
        }
    }

    fn add_step(&mut self, step: &str) {
        self.steps.push(step.to_string());
    }

    fn run(&self) {
        println!("Running pipeline: {}", self.name);
        for step in &self.steps {
            println!("Executing step: {}", step);
        }
    }
}

fn main() {
    let mut ml_pipeline = Pipeline::new("Distributed Training");
    ml_pipeline.add_step("Data Ingestion");
    ml_pipeline.add_step("Preprocessing");
    ml_pipeline.add_step("Model Training");
    ml_pipeline.add_step("Evaluation");
    ml_pipeline.run();
}
