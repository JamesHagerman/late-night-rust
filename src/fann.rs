extern crate fann;
use fann::{ActivationFunc, Fann, TrainAlgorithm, QuickpropParams};

pub fn run() {
   // Create a new network with two input neurons, a hidden layer with three neurons, and one
   // output neuron.
   let mut fann = Fann::new(&[2, 3, 1]).unwrap();
   // Configure the activation functions for the hidden and output neurons.
   fann.set_activation_func_hidden(ActivationFunc::SigmoidSymmetric);
   fann.set_activation_func_output(ActivationFunc::SigmoidSymmetric);
   // Use the Quickprop learning algorithm, with default parameters.
   // (Otherwise, Rprop would be used.)
   fann.set_train_algorithm(TrainAlgorithm::Quickprop(Default::default()));
   // Train for up to 500000 epochs, displaying progress information after intervals of 1000
   // epochs. Stop when the network's error on the training data drops to 0.001.
   let max_epochs = 500000;
   let epochs_between_reports = 1000;
   let desired_error = 0.001;
   // Train directly on data loaded from the file "xor.data".
   fann.on_file("test_files/xor.data")
       .with_reports(epochs_between_reports)
       .train(max_epochs, desired_error).unwrap();
   // The network now approximates the XOR problem:
   assert!(fann.run(&[-1.0,  1.0]).unwrap()[0] > 0.9);
   assert!(fann.run(&[ 1.0, -1.0]).unwrap()[0] > 0.9);
   assert!(fann.run(&[ 1.0,  1.0]).unwrap()[0] < 0.1);
   assert!(fann.run(&[-1.0, -1.0]).unwrap()[0] < 0.1);
}
