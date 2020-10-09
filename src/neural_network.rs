use rand::Rng;

pub struct NeuralNetwork {
    dims: Vec<usize>,
    weights: Vec<Vec<Vec<f32>>>
}

impl NeuralNetwork {
    pub fn new(dims: &Vec<usize>) -> NeuralNetwork {
        NeuralNetwork {
            dims: dims.clone(),
            weights: generate_random_weights(dims)
        }
    }
}

fn generate_random_weights(dims: &Vec<usize>) -> Vec<Vec<Vec<f32>>> {
    let mut rng = rand::thread_rng();

    let mut random_weights: Vec<Vec<Vec<f32>>> = vec![];

    let next_dims = &dims[1..];
    for (&prev_dim, &current_dim) in dims.iter().zip(next_dims.iter()) {
        let mut layer_weights: Vec<Vec<f32>> = vec![];

        for _ in 0..current_dim {
            let mut node_weights: Vec<f32> = vec![];

            for _ in 0..prev_dim {
                node_weights.push(rng.gen_range(-0.1, 0.1));
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

    for &layer_dim in dims {
        let mut layer_biases = vec![];

        for _ in 0..layer_dim {
            layer_biases.push(rng.gen_range(-0.1, 1.0));
        }

        random_biases.push(layer_biases);
    }

    random_biases
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
            let prev_dim = dims[i-1];

            let current_weights = &random_weights[i-1];

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

        for (index, &dim) in dims.iter().enumerate() {
            assert_eq!(dim, random_biases[index].len());
        }

    }
}
