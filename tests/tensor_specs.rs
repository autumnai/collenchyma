extern crate collenchyma as co;

#[cfg(test)]
mod tensor_spec {

    use co::shared_memory::*;

    #[test]
    fn it_works_for_a_tensorr0() {
        let tensor = TensorR0::new();
        assert_eq!(0, TensorR0::D());
        assert_eq!(0, tensor.dims().len());
        assert_eq!(1, tensor.elements());
    }

    #[test]
    fn it_works_for_a_tensorr1() {
        let tensor = TensorR1::new([5]);
        assert_eq!(1, TensorR1::D());
        assert_eq!(1, tensor.dims().len());
        assert_eq!(5, tensor.elements());
    }

    #[test]
    fn it_works_for_a_tensorr2() {
        let tensor = TensorR2::new([5, 5]);
        assert_eq!(2, TensorR2::D());
        assert_eq!(2, tensor.dims().len());
        assert_eq!(25, tensor.elements());
    }

    #[test]
    fn it_works_for_a_tensorr3() {
        let tensor = TensorR3::new([5, 5, 5]);
        assert_eq!(3, TensorR3::D());
        assert_eq!(3, tensor.dims().len());
        assert_eq!(125, tensor.elements());
    }

    #[test]
    fn it_works_for_a_tensorr4() {
        let tensor = TensorR4::new([5, 5, 5, 5]);
        assert_eq!(4, TensorR4::D());
        assert_eq!(4, tensor.dims().len());
        assert_eq!(625, tensor.elements());
    }

    #[test]
    fn it_works_for_a_tensorr5() {
        let tensor = TensorR5::new([5, 5, 5, 5, 5]);
        assert_eq!(5, TensorR5::D());
        assert_eq!(5, tensor.dims().len());
        assert_eq!(3125, tensor.elements());
    }

    #[test]
    fn it_works_for_a_tensorr6() {
        let tensor = TensorR6::new([5, 5, 5, 5, 5, 5]);
        assert_eq!(6, TensorR6::D());
        assert_eq!(6, tensor.dims().len());
        assert_eq!(15625, tensor.elements());
    }
}
