use anchor_lang::error_code;

#[error_code]
pub enum DeliError {
    #[msg("Product name is too long")]
    ProductNameTooLong,
    #[msg("Tier name is too long")]
    TierNameTooLong,
    #[msg("Not enough funds to reserve subscription")]
    InsufficientFunds,
    #[msg("Subscription is not due for collection yet")]
    SubscriptionNotDue,
}
