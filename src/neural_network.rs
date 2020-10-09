pub struct NeuralNetwork {
    dims: Vec<usize>
}

impl NeuralNetwork {
    pub fn new(dims: &Vec<usize>) -> NeuralNetwork {
        NeuralNetwork {
            dims: dims.clone()
        }
    }
}
