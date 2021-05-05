pub trait OrderInterface<T: frame_system::Config> {
    type Order;
    type Error;

    //fn generate_order_id(customer_id: &T::AccountId, service_id: &T::Hash) -> T::Hash;
    fn create_order(customer_id: &T::AccountId, service_id: &T::Hash) -> Result<Self::Order, Self::Error>;
    // set_order_paid Should only be called by Escrow API Server with the correct account_id
    fn set_order_paid(escrow_account_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error>;
    fn fulfill_order(seller_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error>;
    fn refund_order(escrow_account_id: &T::AccountId, order_id: &T::Hash) -> Result<Self::Order, Self::Error>;

    /*
    fn order_by_id(order_id: &T::Hash) -> Option<Self::Order>;
    fn orders_by_seller_id(seller_id: &T::AccountId) -> Vec<T::Hash>;
    fn orders_by_customer_id(customer_id: &T::AccountId) -> Vec<T::Hash>;
    */
}
