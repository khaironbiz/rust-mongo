pub mod auth_handlers;
pub mod user_handlers;
pub mod medical_record_handlers;
pub mod doctor_handlers;
pub mod nurse_handlers;
pub mod medicine_handlers;
pub mod appointment_handlers;
pub mod service_handlers;
pub mod insurance_handlers;
pub mod file_handlers;

pub mod role_handlers;
pub mod user_role_handlers;
pub mod child_code_handlers;
pub mod region_handlers;

// Re-export handlers for easy access
pub use auth_handlers::*;
pub use user_handlers::*;
pub use medical_record_handlers::*;
pub use doctor_handlers::*;
pub use nurse_handlers::*;
pub use medicine_handlers::*;
pub use appointment_handlers::*;
pub use service_handlers::*;
pub use insurance_handlers::*;
pub use file_handlers::*;
pub use role_handlers::*;
pub use user_role_handlers::*;
pub use code_handlers::*;

pub mod code_handlers;
