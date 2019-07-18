pub trait Table<R> {
    fn get_table(&self) -> Vec<&R>;

    fn get_record(&self, pos: usize) -> Option<&R>;
}
