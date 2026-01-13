pub mod tenants;
pub mod databases;

pub use tenants::tenant::ActiveModel as TenantAM;
pub use databases::database::ActiveModel as DatabaseAM;

pub use tenants::tenant::Model as Tenant;
pub use databases::database::Model as Database;

pub use tenants::tenant::Entity as TenantEntity;
pub use databases::database::Entity as DatabaseEntity;

pub use tenants::repository as TenantRepo;

//Tenant - Tenant, Database - Database