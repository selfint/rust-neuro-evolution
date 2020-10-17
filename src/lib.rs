mod environment;
mod evolution;
mod neural_network;

#[cfg(test)]
mod tests {
    mod neural_network_integration_tests {
        use crate::neural_network::NeuralNetwork;

        #[test]
        fn neural_network_exists() {
            let network_dims = [2, 3, 1];
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        fn constructor_borrows_dims() {
            let network_dims = [2, 3, 1];
            NeuralNetwork::new(&network_dims);
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        #[should_panic(expected = "Neural network dims must have at least 2 layers, got 1 layers")]
        fn constructor_panics_on_not_enough_layers() {
            let network_dims = [2];
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        #[should_panic(
            expected = "Given input has length 1, while network expects input with length 3"
        )]
        fn feed_forward_panics_on_wrong_input_size() {
            let nn = NeuralNetwork::new(&[3, 10, 1]);

            nn.feed_forward(&[1.0]);
        }

        #[test]
        fn feed_forward_gives_correct_output_length() {
            let nn = NeuralNetwork::new(&[2, 1, 1]);
            let output: Vec<f32> = nn.feed_forward(&[1.0, 0.0]);

            assert_eq!(1, output.len());
        }

        #[test]
        fn total_weights_is_correct() {
            let nn = NeuralNetwork::new(&[2, 3, 1]);

            assert_eq!(9, nn.total_weights());
        }
    }

    mod evolution_integration_tests {
        use crate::evolution;
        use crate::neural_network::NeuralNetwork;

        #[test]
        fn mutate_changes_one_weight_or_bias() {
            let mut nn = NeuralNetwork::new(&[2, 4, 3]);
            let nn_weights = nn.weights.clone();
            let nn_biases = nn.biases.clone();

            evolution::mutate(&mut nn);

            let mutated_weights = nn.weights.clone();
            let mutated_biases = nn.biases.clone();

            let changed_weights =
                networks_weights_diff_count(&nn_weights, &mutated_weights).unwrap();
            let changed_biases = networks_bias_diff_count(&nn_biases, &mutated_biases).unwrap();

            assert_eq!(1, changed_weights + changed_biases);
        }

        fn networks_weights_diff_count(
            nn1_network_weights: &Vec<Vec<Vec<f32>>>,
            nn2_network_weights: &Vec<Vec<Vec<f32>>>,
        ) -> Result<u32, String> {
            let mut diff_counter = 0;
            for (nn1_layer, nn2_layer) in nn1_network_weights.iter().zip(nn2_network_weights.iter())
            {
                if nn1_layer.len() != nn2_layer.len() {
                    return Err("Network dim mismatch".to_owned());
                }

                for (nn1_weights, nn2_weights) in nn1_layer.iter().zip(nn2_layer.iter()) {
                    if nn1_weights.len() != nn2_weights.len() {
                        return Err("Network dim mismatch".to_owned());
                    }

                    for (&nn1_weight, &nn2_weight) in nn1_weights.iter().zip(nn2_weights.iter()) {
                        if nn1_weight != nn2_weight {
                            diff_counter += 1;
                        }
                    }
                }
            }

            Ok(diff_counter)
        }

        fn networks_bias_diff_count(
            nn1_network_biases: &Vec<Vec<f32>>,
            nn2_network_biases: &Vec<Vec<f32>>,
        ) -> Result<u32, String> {
            let mut diff_counter = 0;
            for (nn1_layer, nn2_layer) in nn1_network_biases.iter().zip(nn2_network_biases.iter()) {
                if nn1_layer.len() != nn2_layer.len() {
                    return Err("Network dim mismatch".to_owned());
                }

                for (&nn1_bias, &nn2_bias) in nn1_layer.iter().zip(nn2_layer.iter()) {
                    if nn1_bias != nn2_bias {
                        diff_counter += 1;
                    }
                }
            }

            Ok(diff_counter)
        }

        #[test]
        #[should_panic(expected = "Can't crossover networks with different dimensions")]
        fn crossover_panics_on_network_dims_mismatch() {
            let nn1 = NeuralNetwork::new(&[2, 3, 1]);
            let nn2 = NeuralNetwork::new(&[1, 4, 2]);

            evolution::crossover(&nn1, &nn2);
        }

        #[test]
        fn crossover_uses_weights_from_both_networks() {
            let network_dims = [2, 3, 1];
            let nn1 = NeuralNetwork::new(&network_dims);
            let nn2 = NeuralNetwork::new(&network_dims);

            let nn3: NeuralNetwork = evolution::crossover(&nn1, &nn2);

            let nn1_weights_diff_count =
                networks_weights_diff_count(&nn3.weights, &nn1.weights).unwrap();
            let nn2_weights_diff_count =
                networks_weights_diff_count(&nn3.weights, &nn2.weights).unwrap();

            let parent_nn_weight_diff_count =
                networks_weights_diff_count(&nn1.weights, &nn2.weights).unwrap();
            assert_ne!(0, parent_nn_weight_diff_count);
            assert_eq!(
                nn3.total_weights(),
                nn1_weights_diff_count + nn2_weights_diff_count
            );
        }

        #[test]
        fn crossover_uses_biases_from_both_networks() {
            let network_dims = [2, 3, 1];
            let nn1 = NeuralNetwork::new(&network_dims);
            let nn2 = NeuralNetwork::new(&network_dims);

            let nn3: NeuralNetwork = evolution::crossover(&nn1, &nn2);

            let nn1_biases_diff_count = networks_bias_diff_count(&nn3.biases, &nn1.biases).unwrap();
            let nn2_biases_diff_count = networks_bias_diff_count(&nn3.biases, &nn2.biases).unwrap();

            let parent_nn_biases_diff_count =
                networks_bias_diff_count(&nn1.biases, &nn2.biases).unwrap();
            assert_ne!(0, parent_nn_biases_diff_count);
            assert_eq!(
                nn3.total_biases(),
                nn1_biases_diff_count + nn2_biases_diff_count
            );
        }

        #[test]
        fn spawn_generation_is_correct_amount() {
            let network_dims = [2, 3, 1];
            let total_networks: usize = 10;
            let networks: Vec<NeuralNetwork> =
                evolution::spawn_generation(total_networks, &network_dims);

            assert_eq!(total_networks, networks.len());
        }

        #[test]
        fn new_generation_is_correct_amount() {
            let network_dims = [2, 3, 1];
            let total_networks: usize = 4;
            let mutation_rate = 1.0;
            let survival_rate = 0.5;
            let networks: Vec<NeuralNetwork> =
                evolution::spawn_generation(total_networks, &network_dims);

            let scores = [1.0, 2.0, 3.0, 4.0];

            let new_networks: Vec<NeuralNetwork> =
                evolution::new_generation(&networks, &scores, mutation_rate, survival_rate);

            assert_eq!(total_networks, new_networks.len());
        }

        #[test]
        fn new_generation_makes_correct_amount_of_new_networks() {
            let network_dims = [2, 3, 1];
            let total_networks: usize = 4;
            let mutation_rate = 1.0;
            let survival_rate = 0.5;
            let networks: Vec<NeuralNetwork> =
                evolution::spawn_generation(total_networks, &network_dims);

            let scores = [1.0, 2.0, 3.0, 4.0];

            let new_networks: Vec<NeuralNetwork> =
                evolution::new_generation(&networks, &scores, mutation_rate, survival_rate);

            let mut total_new_networks = new_networks.len();
            for new_network in &new_networks {
                for old_network in &networks {
                    let weight_diffs =
                        networks_weights_diff_count(&old_network.weights, &new_network.weights)
                            .unwrap();
                    let bias_diffs =
                        networks_bias_diff_count(&old_network.biases, &new_network.biases).unwrap();

                    if weight_diffs + bias_diffs == 0 {
                        total_new_networks -= 1;
                        break;
                    }
                }
            }

            assert_eq!(2, total_new_networks);
        }

        #[test]
        fn new_generation_keeps_correct_amount_of_old_networks() {
            let network_dims = [2, 3, 1];
            let total_networks = 4;
            let mutation_rate = 1.0;
            let survival_rate = 0.5;
            let scores = [1.0, 2.0, 3.0, 4.0];

            let networks = evolution::spawn_generation(total_networks, &network_dims);

            let next_generation =
                evolution::new_generation(&networks, &scores, mutation_rate, survival_rate);

            let mut survivor_count = 0;
            for old_network in &networks {
                for new_network in &next_generation {
                    let weight_diffs =
                        networks_weights_diff_count(&old_network.weights, &new_network.weights)
                            .unwrap();
                    let bias_diffs =
                        networks_bias_diff_count(&old_network.biases, &new_network.biases).unwrap();

                    if weight_diffs + bias_diffs == 0 {
                        survivor_count += 1;
                    }
                }
            }

            assert_eq!(2, survivor_count);
        }
    }

    mod environment_integration_tests {
        use crate::environment::Bounce;

        #[test]
        fn test() {
            println!("hello!");
        }
    }
}
