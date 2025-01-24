mod pipeline;
use pipeline::MLOpsPipeline;

fn main() {
    let mut pipeline = MLOpsPipeline::new("Production-Inference-Pipeline");
    pipeline.set_config("batch_size", "32");
    pipeline.set_config("target_device", "cuda:0");

    match pipeline.execute() {
        Ok(_) => println!("Pipeline execution completed successfully."),
        Err(e) => eprintln!("Pipeline failed: {}", e),
    }
}
