pub trait ColyaResource {
    fn id(&self) -> u64;
    fn balance(&self) -> u128;
    // Consumes self: The variable is GONE after this call
    fn move_resource(self, to: String) -> Self;
}
