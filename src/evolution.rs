use crate::neural_network;
use rand::Rng;

pub fn mutate(nn: &mut neural_network::NeuralNetwork) {
    let mut rng = rand::thread_rng();

    let chosen_layer = rng.gen_range(0, nn.weights.len());
    let chosen_node = rng.gen_range(0, nn.weights[chosen_layer].len());
    let chosen_weight = rng.gen_range(0, nn.weights[chosen_layer][chosen_node].len());

    nn.weights[chosen_layer][chosen_node][chosen_weight] = neural_network::generate_random_weight(rng);
}