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

    }
}
