use crate::neural_network;
use crate::neural_network::NeuralNetwork;
use rand::Rng;

pub fn mutate(nn: &mut NeuralNetwork) {
    let mut rng = rand::thread_rng();

    let chosen_layer = rng.gen_range(0, nn.weights.len());
    let chosen_node = rng.gen_range(0, nn.weights[chosen_layer].len());
    let chosen_weight = rng.gen_range(0, nn.weights[chosen_layer][chosen_node].len());

    nn.weights[chosen_layer][chosen_node][chosen_weight] =
        neural_network::generate_random_weight(rng);
}

pub fn crossover(nn1: &NeuralNetwork, nn2: &NeuralNetwork) -> NeuralNetwork {
    if nn1.dims != nn2.dims {
        panic!("Can't crossover networks with different dimensions");
    }

    let nn3_weights = crossover_weights(nn1, nn2);
    let nn3_biases = crossover_biases(nn1, nn2);

    let nn3 = NeuralNetwork {
        dims: nn1.dims.clone(),
        inputs: nn1.inputs,
        outputs: nn1.outputs,
        biases: nn3_biases,
        weights: nn3_weights
    };

    nn3
}

fn crossover_biases(nn1: &NeuralNetwork, nn2: &NeuralNetwork) -> Vec<Vec<f32>> {
    let mut nn3_network_biases = vec![];
    for (nn1_layer, nn2_layer) in nn1.biases.iter().zip(nn2.biases.iter()) {
        let mut nn3_layer = vec![];
        for (&nn1_bias, &nn2_bias) in nn1_layer.iter().zip(nn2_layer.iter()) {
            if rand::random() {
                nn3_layer.push(nn1_bias);
            } else {
                nn3_layer.push(nn2_bias);
            }
        }

        nn3_network_biases.push(nn3_layer);
    }

    nn3_network_biases
}

fn crossover_weights(nn1: &NeuralNetwork, nn2: &NeuralNetwork) -> Vec<Vec<Vec<f32>>> {
    let mut nn3_network_weights = vec![];
    for (nn1_layer, nn2_layer) in nn1.weights.iter().zip(nn2.weights.iter()) {
        let mut nn3_layer = vec![];
        for (nn1_weights, nn2_weights) in nn1_layer.iter().zip(nn2_layer.iter()) {
            let mut nn3_weights = vec![];
            for (&nn1_weight, &nn2_weight) in nn1_weights.iter().zip(nn2_weights.iter()) {
                if rand::random() {
                    nn3_weights.push(nn1_weight);
                } else {
                    nn3_weights.push(nn2_weight);
                }
            }

            nn3_layer.push(nn3_weights)
        }

        nn3_network_weights.push(nn3_layer)
    }

    nn3_network_weights
}
