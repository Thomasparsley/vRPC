pub trait AppInfoLike: Sized {
    fn name(&self) -> String;
    fn version(&self) -> String;
    fn description(&self) -> String;
}
