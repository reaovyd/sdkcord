use bon::{Builder, builder};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Builder)]
pub struct Pan {
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    pub left: OrderedFloat<f32>,
    #[builder(with = |x: f32| {
        OrderedFloat(x)
    })]
    pub right: OrderedFloat<f32>,
}

impl Pan {
    pub const fn left(&self) -> f32 {
        self.left.0
    }

    pub const fn right(&self) -> f32 {
        self.right.0
    }
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
