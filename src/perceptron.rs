use ndarray::{Array1, Array2, Axis};
use num_traits::Float;

pub struct Perceptron<F: Float> {
    //    pub(crate) b: Array1<F>,
    pub(crate) w: Array2<F>,
}

impl<F: Float> Perceptron<F> {
    pub fn output(&self, input: &Array1<F>) -> Array1<F> {
        let input2d = input.view().insert_axis(Axis(0));
        let a1 = &input2d * &self.w;
        a1.sum_axis(Axis(0))
    }
}

#[cfg(test)]
mod test {
    use ndarray::array;

    use crate::perceptron::Perceptron;

    #[test]
    fn p_1() {
        let ps: Perceptron<f32> = Perceptron { w: array![[2.0]] };
        let i = array![3.0];

        assert_eq!(ps.output(&i), array![6.0]);
    }

    #[test]
    fn p_2() {
        let ps: Perceptron<f32> = Perceptron {
            w: array![[1.0, 2.0], [3.0, 4.0]],
        };
        let i = array![1.0, 2.0];
        let expected = array![4.0, 12.0];

        assert_eq!(ps.output(&i), expected);
    }
}
