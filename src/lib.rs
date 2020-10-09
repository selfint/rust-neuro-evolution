mod neural_network;

#[cfg(test)]
mod tests {
    use super::neural_network::NeuralNetwork;

    #[test]
    fn neural_network_exists() {
        let network_dims = vec![2, 3, 1];
        let x = NeuralNetwork::new(&network_dims);
    }

    #[test]
    fn constructor_doesnt_consume_dims() {
        let network_dims = vec![2, 3, 1];
        let x = NeuralNetwork::new(&network_dims);
        let y = NeuralNetwork::new(&network_dims);
    }
}
