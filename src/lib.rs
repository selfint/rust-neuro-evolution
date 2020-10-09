mod neural_network;

#[cfg(test)]
mod tests {
    use crate::neural_network::NeuralNetwork;

    #[test]
    fn neural_network_exists() {
        let x = NeuralNetwork::new();
    }
}
