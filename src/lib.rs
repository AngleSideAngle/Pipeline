pub mod filter;
pub mod junction;
pub mod stream;

#[cfg(test)]
mod tests {
    use crate::stream::Stream;

    use super::*;

    #[test]
    fn intergation() {
        let one = Stream::new(|| 1.0);
        let x = one.clone().integrate(Some(1.0));
        assert_eq!(x.get(), 1.0);
        assert_eq!(x.get(), 2.0);
    }

    #[test]
    fn differentiation() {
        let one = Stream::new(|| 1.0);
        let zero = one.clone().differentiate(None);
        assert_eq!(zero.get(), 0.0);
        assert_eq!(zero.get(), 0.0);
    }

    #[test]
    fn inverses() {
        let one = Stream::new(|| 1.0);
        let other = one.clone().integrate(Some(1.0));
        let other = other.differentiate(Some(1.0));
        other.get();
        assert_eq!(one.get(), other.get());
        assert_eq!(one.get(), other.get());
    }
}
