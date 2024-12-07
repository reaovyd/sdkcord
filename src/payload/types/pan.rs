use bon::{builder, Builder};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Pan {
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    left: OrderedFloat<f32>,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    right: OrderedFloat<f32>,
}

#[cfg(test)]
mod tests {
    use super::Pan;

    #[test]
    fn construct_pan() {
        let pan = Pan::builder().left(23.0).right(10.0).build();
        assert_eq!(pan.left.0, 23.0);
        assert_eq!(pan.right.0, 10.0);
    }
}
