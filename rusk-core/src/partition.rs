// Core trait for data partitioning
pub trait Partition<T>: Send + Sync {
    fn id(&self) -> usize;
    fn data(&self) -> &[T];
    fn size(&self) -> usize;
    fn split(self, num_partitions: usize) -> Vec<Box<dyn Partition<T>>>;
}

// Concrete implementations
pub struct MemoryPartition<T> { /* ... */ }
pub struct FilePartition<T> { /* ... */ }

// Key functions
impl<T> MemoryPartition<T> {
    pub fn new(id: usize, data: Vec<T>) -> Self
    pub fn from_slice(id: usize, data: &[T]) -> Self where T: Clone
}
