use rmic::Gmic;

fn main() {
    let input_file = "input.jpg";
    let output_file = "result.jpg";

    let gmic_task = Gmic::new()
        .input(input_file)
        .watermark("rmic +  G'MIC", 0.9, 53, 25, 0)
        .output(output_file);

    match gmic_task.execute() {
        Ok(_) => println!("Success! Image saved to {}", output_file),
        Err(e) => eprintln!("Error running G'MIC: {:?}", e),
    }
}
