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
        #[should_panic]
        fn constructor_panics_on_not_enough_layers() {
            let network_dims = vec![2];
            NeuralNetwork::new(&network_dims);
        }

        #[test]
        #[should_panic]
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
}
