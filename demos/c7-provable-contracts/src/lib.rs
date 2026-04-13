//! Fruit processing pipeline with compile-time typestate guarantees.
//!
//! State machine: `Configured -> Validated -> Processing -> Complete`
//! Invalid transitions are compile errors — the methods simply don't exist.

use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error("price must be greater than zero")]
    ZeroPrice,
    #[error("batch must not be empty")]
    EmptyBatch,
}

// ---------------------------------------------------------------------------
// Newtypes
// ---------------------------------------------------------------------------

/// Monetary amount in cents. Zero is rejected at construction time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceCents(u64);

impl PriceCents {
    pub fn new(value: u64) -> Result<Self, PipelineError> {
        if value == 0 {
            return Err(PipelineError::ZeroPrice);
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

/// Item count. Zero is perfectly valid (e.g., sold-out fruit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity(u32);

impl Quantity {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FruitItem {
    pub name: String,
    pub price: PriceCents,
    pub qty: Quantity,
}

#[derive(Debug, Clone)]
pub struct FruitBatch {
    pub items: Vec<FruitItem>,
}

impl FruitBatch {
    pub fn total_items(&self) -> u64 {
        self.items.iter().map(|i| u64::from(i.qty.value())).sum()
    }

    pub fn total_revenue(&self) -> u64 {
        self.items
            .iter()
            .map(|i| i.price.value() * u64::from(i.qty.value()))
            .sum()
    }
}

// ---------------------------------------------------------------------------
// Typestate markers (ZSTs)
// ---------------------------------------------------------------------------

pub struct Configured;
pub struct Validated;
pub struct Processing;
pub struct Complete;

// ---------------------------------------------------------------------------
// Pipeline
// ---------------------------------------------------------------------------

pub struct Pipeline<S> {
    batch: FruitBatch,
    _state: PhantomData<S>,
}

// --- Configured -----------------------------------------------------------

impl Pipeline<Configured> {
    pub fn new(batch: FruitBatch) -> Self {
        Self {
            batch,
            _state: PhantomData,
        }
    }

    /// Transition to `Validated`. Fails if the batch is empty.
    pub fn validate(self) -> Result<Pipeline<Validated>, PipelineError> {
        if self.batch.items.is_empty() {
            return Err(PipelineError::EmptyBatch);
        }
        Ok(Pipeline {
            batch: self.batch,
            _state: PhantomData,
        })
    }
}

// --- Validated ------------------------------------------------------------

impl Pipeline<Validated> {
    /// Apply a 10 % discount (price * 90 / 100). Quantities never change.
    pub fn process(self) -> Pipeline<Processing> {
        let items = self
            .batch
            .items
            .into_iter()
            .map(|mut item| {
                let discounted = item.price.value() * 90 / 100;
                // Discount can reduce a 1-cent price to 0; clamp to 1.
                item.price = PriceCents(discounted.max(1));
                item
            })
            .collect();
        Pipeline {
            batch: FruitBatch { items },
            _state: PhantomData,
        }
    }
}

// --- Processing -----------------------------------------------------------

impl Pipeline<Processing> {
    pub fn complete(self) -> Pipeline<Complete> {
        Pipeline {
            batch: self.batch,
            _state: PhantomData,
        }
    }
}

// --- Complete -------------------------------------------------------------

impl Pipeline<Complete> {
    pub fn into_batch(self) -> FruitBatch {
        self.batch
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // --- unit tests -------------------------------------------------------

    #[test]
    fn price_zero_is_rejected() {
        assert_eq!(PriceCents::new(0), Err(PipelineError::ZeroPrice));
    }

    #[test]
    fn price_nonzero_ok() {
        assert!(PriceCents::new(42).is_ok());
    }

    #[test]
    fn quantity_zero_is_valid() {
        assert_eq!(Quantity::new(0).value(), 0);
    }

    #[test]
    fn empty_batch_fails_validation() {
        let pipeline = Pipeline::new(FruitBatch { items: vec![] });
        assert!(matches!(
            pipeline.validate(),
            Err(PipelineError::EmptyBatch)
        ));
    }

    #[test]
    fn single_item_pipeline_completes() {
        let batch = FruitBatch {
            items: vec![FruitItem {
                name: "Apple".into(),
                price: PriceCents::new(100).unwrap(),
                qty: Quantity::new(5),
            }],
        };
        let result = Pipeline::new(batch)
            .validate()
            .unwrap()
            .process()
            .complete()
            .into_batch();
        assert_eq!(result.total_items(), 5);
    }

    // --- property tests ---------------------------------------------------

    prop_compose! {
        fn arb_fruit_item()(
            name in "[a-z]{3,8}",
            price in 1_u64..10_000,
            qty in 0_u32..1_000,
        ) -> FruitItem {
            FruitItem {
                name,
                price: PriceCents::new(price).unwrap(),
                qty: Quantity::new(qty),
            }
        }
    }

    prop_compose! {
        fn arb_nonempty_batch()(items in prop::collection::vec(arb_fruit_item(), 1..20))
            -> FruitBatch
        {
            FruitBatch { items }
        }
    }

    proptest! {
        /// Pipeline preserves total item count (no fruit lost).
        #[test]
        fn pipeline_preserves_total_items(batch in arb_nonempty_batch()) {
            let before = batch.total_items();
            let after = Pipeline::new(batch)
                .validate()
                .unwrap()
                .process()
                .complete()
                .into_batch()
                .total_items();
            prop_assert_eq!(before, after);
        }

        /// Revenue equals the sum of price * qty for every item.
        #[test]
        fn revenue_equals_sum_of_price_times_qty(batch in arb_nonempty_batch()) {
            let expected: u64 = batch
                .items
                .iter()
                .map(|i| i.price.value() * u64::from(i.qty.value()))
                .sum();
            prop_assert_eq!(batch.total_revenue(), expected);
        }
    }
}
