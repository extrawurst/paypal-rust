use serde::{Deserialize, Serialize};

/// The person or party who initiated or triggered the payment.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum PaymentInitiator {
    /// Payment is initiated with the active engagement of the customer. e.g. a customer checking out on a merchant website.
    #[serde(rename = "CUSTOMER")]
    Customer,
    #[serde(rename = "MERCHANT")]
    /// Payment is initiated by merchant on behalf of the customer without the active engagement of customer. e.g. a merchant
    /// charging the monthly payment of a subscription to the customer.
    Merchant,
}

impl PaymentInitiator {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentInitiator::Customer => "CUSTOMER",
            PaymentInitiator::Merchant => "MERCHANT",
        }
    }
}

impl Default for PaymentInitiator {
    fn default() -> Self {
        PaymentInitiator::Customer
    }
}

impl AsRef<str> for PaymentInitiator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentInitiator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(formatter)
    }
}
