pub mod medical_record;
pub mod file;
pub mod doctor;
pub mod nurse;
pub mod medicine;
pub mod appointment;
pub mod service;
pub mod insurance;
pub mod user;

pub use medical_record::MedicalRecordRepository;
pub use file::FileRepository;
pub use doctor::DoctorRepository;
pub use nurse::NurseRepository;
pub use medicine::MedicineRepository;
pub use appointment::AppointmentRepository;
pub use service::ServiceRepository;
pub use insurance::InsuranceRepository;
pub use user::UserRepository;
pub use role::RoleRepository;
pub use user_role::UserRoleRepository;

pub mod role;
pub mod user_role;
pub mod child_code;
pub mod code;

pub use code::CodeRepository;
pub use child_code::ChildCodeRepository;
pub mod region;
pub use region::RegionRepository;
pub mod interpretation;
pub use interpretation::InterpretationRepository;
pub mod kit;
pub use kit::KitRepository;
pub mod observation;
pub use observation::ObservationRepository;
