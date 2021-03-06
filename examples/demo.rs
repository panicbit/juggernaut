extern crate juggernaut;

use juggernaut::nl::NeuralLayer;
use juggernaut::nn::NeuralNetwork;
use juggernaut::activation::Activation;
use juggernaut::activation::Sigmoid;
use juggernaut::sample::Sample;

fn main() {
    println!("Juggernaut...");

    let dataset = vec![
        Sample::new(vec![0f64, 0f64, 1f64], vec![0f64]),
        Sample::new(vec![0f64, 1f64, 1f64], vec![0f64]),
        Sample::new(vec![1f64, 0f64, 1f64], vec![1f64]),
        Sample::new(vec![1f64, 1f64, 1f64], vec![1f64])
    ];

    let think_dataset = vec![
        Sample::new(vec![1f64, 0f64, 1f64], vec![0f64])
    ];

    println!("Creating the network...");

    let mut test = NeuralNetwork::new(dataset, Sigmoid::new());

    // 1st layer = 2 neurons - 3 inputs
    test.add_layer(NeuralLayer::new(2, 3));

    println!("First layer created: 2 neurons 3 inputs");

    // 2nd layer = 1 neuron - 2 inputs
    test.add_layer(NeuralLayer::new(1, 2));

    println!("Second layer created: 1 neuron 2 inputs");

    println!("Training (60,000 epochs)...");

    test.train(60000);

    println!("Done!!");

    let think = test.forward(&think_dataset);

    println!("{:?}", think);
}
