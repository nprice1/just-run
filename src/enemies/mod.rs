// Bring enemies into this crate's namespace
pub use game::enemies::zombie::SlowZombie;
pub use game::enemies::zombie::CrazyZombie;
pub use game::enemies::zombie::RandomZombie;
pub use game::enemies::zombie::CloudZombie;
pub use game::enemies::zombie::Zombie;

// Load enemy modules
pub mod zombie;
