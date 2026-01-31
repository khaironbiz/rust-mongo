pub mod medical_record_service;
pub mod file_service;
pub mod doctor_service;
pub mod nurse_service;
pub mod medicine_service;
pub mod appointment_service;
pub mod service_service;
pub mod insurance_service;
pub mod auth_service;
pub mod user_service;

pub use medical_record_service::MedicalRecordService;
pub use file_service::FileService;
pub use doctor_service::DoctorService;
pub use nurse_service::NurseService;
pub use medicine_service::MedicineService;
pub use appointment_service::AppointmentService;
pub use service_service::ServiceService;
pub use insurance_service::InsuranceService;
pub use auth_service::AuthService;
pub use user_service::UserService;
pub use role_service::RoleService;
pub use user_role_service::UserRoleService;

pub mod role_service;
pub mod user_role_service;
pub mod child_code_service;
pub mod code_service;

pub use code_service::CodeService;
pub use child_code_service::ChildCodeService;
pub mod region_service;
pub use region_service::RegionService;
