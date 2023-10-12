use anchor_lang::error_code;

#[error_code]
pub enum SubscriptionError {
    #[msg("Not enough funds to deleate for subscription")]
    InsufficientFunds,
    #[msg("Subscription is not due for collection yet")]
    SubscriptionNotDue,
}
