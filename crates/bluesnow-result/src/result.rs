use crate::Error;
use entity::utils::Resource;

pub type Result<T> = std::result::Result<T, super::Error>;

pub trait ToResult {
    type Output: entity::utils::AppEntity;
    fn to_result(self) -> Result<Self::Output>;
    fn check_absent(self) -> Result<()>;
    fn check_absent_details(self, details: Vec<(String, String)>) -> Result<()>;
    fn to_result_details(self, details: Vec<(String, String)>) -> Result<Self::Output>;
}

impl<T> ToResult for Option<T>
where
    T: entity::utils::AppEntity,
{
    type Output = T;
    fn to_result(self) -> Result<Self::Output> {
        self.ok_or_else(|| {
            Error::NotFound(Resource {
                details: vec![],
                resource_type: Self::Output::RESOURCE,
            })
        })
    }
    fn check_absent(self) -> Result<()> {
        if self.is_some() {
            Err(Error::ResourceExist(Resource {
                details: vec![],
                resource_type: Self::Output::RESOURCE,
            }))
        } else {
            Ok(())
        }
    }

    fn check_absent_details(self, details: Vec<(String, String)>) -> Result<()> {
        if self.is_some() {
            Err(Error::ResourceExist(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            }))
        } else {
            Ok(())
        }
    }
    fn to_result_details(self, details: Vec<(String, String)>) -> Result<Self::Output> {
        self.ok_or_else(|| {
            Error::NotFound(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            })
        })
    }
}
