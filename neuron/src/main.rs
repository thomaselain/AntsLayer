use std::fmt::Debug;
pub const BRAIN_SIZE: usize = 3;

fn main() {
    let colony_needs = Neuron::new(0.0001, 0.3);
    let own_needs = Neuron::new(0.12, 0.15);
    let affinity = Neuron::new(0.78, 0.9);

    let neurons: [Neuron; BRAIN_SIZE] = [colony_needs, own_needs, affinity];
    let brian = Brain { neurons };
    println!("Cerveau en erruption !\n{:?}", brian.process(vec![0.001, 1.0, 0.5]));
}

impl Debug for Brain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Brain").field("neurons", &self.neurons).finish()
    }
}

// #[derive(Debug)]
///
/// A Brain is an array of Neurons
/// giving it a value will make some calculations and give a normalized result based on the number of neurons
/// 
/// 
pub struct Brain {
    neurons: [Neuron; BRAIN_SIZE],
}

impl Brain {
    /// For each neuron in this brain, add   n.a * n.b
    fn process(self, inputs: Vec<f64>) -> Vec<f64> {
        let mut v: Vec<f64> = Vec::new();
        for mut i in inputs {
            for n in self.neurons {
                i += n.a * n.b;
            }
            i /= BRAIN_SIZE as f64;
            v.push(i);
            // println!("Brain did its thing, output is ({})", i);
        }
        v
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
struct Neuron {
    /// Will be added to result
    a: f64,
    /// Will multiply result
    b: f64, 
}

// impl Default for Neuron {
//     fn default() -> Self {
//         Self::new(0.0, 1.0)
//     }
// }

impl Neuron where Neuron: Sized {
    pub fn new(a: f64, b: f64) -> Self {
        let n = Self { a, b };

        n
    }
}
