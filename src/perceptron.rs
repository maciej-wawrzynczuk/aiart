use ndarray::{Array1, Array2, Axis};
use num_traits::Float;

pub struct Perceptron<F: Float> {
    pub(crate) b: Array1<F>,
    pub(crate) w: Array2<F>,
}

impl<F: Float> Perceptron<F> {
    fn intermed(&self, input: &Array1<F>) -> Array1<F> {
        let input2d = input.view().insert_axis(Axis(0));
        let a1 = &input2d * &self.w;
        let weighted = a1.sum_axis(Axis(0));
        &weighted + &self.b
    }

    pub fn output(&self, input: &Array1<F>) -> Array1<F> {
        self.intermed(input).mapv(|x| sigmoid(x))
    }
}

fn sigmoid<F: Float>(x: F) -> F {
    F::one() / (F::one() + (-x).exp())
}

#[cfg(test)]
mod test {
    use ndarray::array;

    use super::*;

    #[test]
    fn p_2() {
        let ps: Perceptron<f32> = Perceptron {
            w: array![[1.0, 2.0], [3.0, 4.0]],
            b: array![10.0, 20.0],
        };
        let i = array![1.0, 2.0];
        let expected = array![14.0, 32.0];

        assert_eq!(ps.intermed(&i), expected);
    }

    #[test]
    fn sigm() {
        assert_eq!(sigmoid(0.0), 0.5);
    }
}
