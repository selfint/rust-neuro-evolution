use rand::Rng;
use rand::prelude::ThreadRng;

pub struct NeuralNetwork {
    pub inputs: usize,
    pub outputs: usize,
    pub weights: Vec<Vec<Vec<f32>>>,
    pub biases: Vec<Vec<f32>>,
}

impl NeuralNetwork {
    pub fn new(dims: &Vec<usize>) -> NeuralNetwork {
        if dims.len() < 2 {
            panic!(format!(
                "Neural network dims must have at least 2 layers, got {} layers",
                dims.len()
            ));
        }

        NeuralNetwork {
            inputs: dims[0],
            outputs: *dims.last().unwrap(),
            weights: generate_random_weights(dims),
            biases: generate_random_biases(dims),
        }
    }

    pub fn feed_forward(&self, input: &Vec<f32>) -> Vec<f32> {
        if self.inputs != input.len() {
            panic!(format!(
                "Given input has length {}, while network expects input with length {}",
                input.len(),
                self.inputs
            ));
        }

        let mut prev_output = input.clone();
        for (layer_weights, layer_biases) in self.weights.iter().zip(self.biases.iter()) {
            let mut output = vec![];
            for (node_weights, node_bias) in layer_weights.iter().zip(layer_biases) {
                let mut node_output = *node_bias;
                for (node_weight, prev_node_output) in node_weights.iter().zip(prev_output.iter()) {
                    node_output += node_weight * prev_node_output;
                }

                output.push(node_output);
            }

            prev_output = output;
        }

        prev_output
    }
}

fn generate_random_weights(dims: &Vec<usize>) -> Vec<Vec<Vec<f32>>> {
    let mut rng = rand::thread_rng();

    let mut random_weights: Vec<Vec<Vec<f32>>> = vec![];

    let next_dims = dims.iter().skip(1);
    for (&prev_dim, &current_dim) in dims.iter().zip(next_dims) {
        let mut layer_weights: Vec<Vec<f32>> = vec![];

        for _ in 0..current_dim {
            let mut node_weights: Vec<f32> = vec![];

            for _ in 0..prev_dim {
                node_weights.push(generate_random_weight(rng));
            }
            layer_weights.push(node_weights);
        }
        random_weights.push(layer_weights);
    }

    random_weights
}

fn generate_random_biases(dims: &Vec<usize>) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();

    let mut random_biases: Vec<Vec<f32>> = vec![];

    // first layer is for inputs only, so no biases
    for &layer_dim in dims.iter().skip(1) {
        let mut layer_biases = vec![];

        for _ in 0..layer_dim {
            layer_biases.push(generate_random_weight(rng));
        }

        random_biases.push(layer_biases);
    }

    random_biases
}

pub fn generate_random_weight(mut rng: ThreadRng) -> f32 {
    rng.gen_range(-0.1, 1.0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn random_weights_have_correct_dims() {
        // must have at least 2 layers
        let dims = vec![2, 3, 1];
        let random_weights: Vec<Vec<Vec<f32>>> = super::generate_random_weights(&dims);

        for i in 1..dims.len() {
            let current_dim = dims[i];
            let prev_dim = dims[i - 1];

            let current_weights = &random_weights[i - 1];

            assert_eq!(current_dim, current_weights.len());

            for node_weights in current_weights {
                assert_eq!(prev_dim, node_weights.len());
            }
        }
    }

    #[test]
    fn random_biases_have_correct_dims() {
        let dims = vec![2, 3, 1];
        let random_biases: Vec<Vec<f32>> = super::generate_random_biases(&dims);

        assert_eq!(3, random_biases[0].len());
        assert_eq!(1, random_biases[1].len());
    }

    #[test]
    fn network_input_outputs_are_correct() {
        let dims = vec![1, 3, 5, 4];
        let nn = super::NeuralNetwork::new(&dims);

        assert_eq!(1, nn.inputs);
        assert_eq!(4, nn.outputs);
    }
}
