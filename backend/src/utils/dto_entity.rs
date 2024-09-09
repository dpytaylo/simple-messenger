pub trait ToDto<T> {
    fn to_dto(&self) -> T;
}

pub trait ToEntity<T> {
    fn to_entity(&self) -> T;
}
