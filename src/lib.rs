mod evolution;
mod neural_network;

#[cfg(test)]
mod tests {
    mod neural_network_integration_tests {
        use crate::neural_network::NeuralNetwork;

        #[test]
        fn neural_network_exists() {
            let network_dims = vec![2, 3, 1];
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        fn constructor_doesnt_consume_dims() {
            let network_dims = vec![2, 3, 1];
            NeuralNetwork::new(&network_dims);
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        #[should_panic(expected = "Neural network dims must have at least 2 layers, got 1 layers")]
        fn constructor_panics_on_not_enough_layers() {
            let network_dims = vec![2];
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        #[should_panic(
            expected = "Given input has length 1, while network expects input with length 3"
        )]
        fn feed_forward_panics_on_wrong_input_size() {
            let nn = NeuralNetwork::new(&vec![3, 10, 1]);

            nn.feed_forward(&vec![1.0]);
        }

        #[test]
        fn feed_forward_gives_correct_output_length() {
            let nn = NeuralNetwork::new(&vec![2, 1, 1]);
            let output: Vec<f32> = nn.feed_forward(&vec![1.0, 0.0]);

            assert_eq!(1, output.len());
        }
    }
    mod evolution_integration_tests {
        use crate::evolution;
        use crate::neural_network::NeuralNetwork;

        #[test]
        fn mutate_changes_one_weight() {
            let mut nn = NeuralNetwork::new(&vec![2, 4, 3]);
            let nn_weights = nn.weights.clone();
            evolution::mutate(&mut nn);
            let mutated_weights = nn.weights.clone();

            let mut diff_counter = 0;
            for (orig_layer, mut_layer) in nn_weights.iter().zip(mutated_weights.iter()) {
                assert_eq!(orig_layer.len(), mut_layer.len());

                for (orig_weights, mut_weights) in orig_layer.iter().zip(mut_layer.iter()) {
                    assert_eq!(orig_weights.len(), mut_weights.len());

                    for (&orig_weight, &mut_weight) in orig_weights.iter().zip(mut_weights.iter()) {
                        if orig_weight != mut_weight {
                            diff_counter += 1;
                        }
                    }
                }
            }

            assert_eq!(1, diff_counter);
        }

        #[test]
        #[should_panic(expected = "Can't crossover networks with different dimensions")]
        fn crossover_panics_on_network_dims_mismatch() {
            let nn1 = NeuralNetwork::new(&vec![2, 3, 1]);
            let nn2 = NeuralNetwork::new(&vec![1, 4, 2]);

            let nn3: NeuralNetwork = evolution::crossover(&nn1, &nn2);
        }
    }
}
