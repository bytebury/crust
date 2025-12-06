use crate::{
    DbPool, DbResult,
    domain::{Country, country::CountryWithRegion},
    infrastructure::{audit::geolocation::CountryDetails, db::CountryRepository},
};

pub struct CountryService {
    country_repository: CountryRepository,
}
impl CountryService {
    pub fn new(db: DbPool) -> Self {
        Self {
            country_repository: CountryRepository::new(db.clone()),
        }
    }

    pub async fn find_by_id(&self, id: i64) -> DbResult<Country> {
        self.country_repository.find_by_id(id).await
    }

    pub async fn find_by_name(&self, name: &str) -> DbResult<Country> {
        self.country_repository.find_by_name(name).await
    }

    pub async fn find_by_code(&self, code: &str) -> DbResult<Country> {
        self.country_repository.find_by_code(code).await
    }

    pub async fn search(&self, value: &str) -> Vec<Country> {
        self.country_repository.search(value).await
    }

    pub async fn lock(&self, id: i64) -> DbResult<()> {
        self.country_repository.lock(id).await
    }

    pub async fn unlock(&self, id: i64) -> DbResult<()> {
        self.country_repository.unlock(id).await
    }

    pub async fn create_or_get(&self, country: &CountryDetails) -> DbResult<CountryWithRegion> {
        self.country_repository.create(country).await
    }
}
