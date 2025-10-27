use serde::Deserialize;
use crate::http_error::{ApiError, FieldError};

#[derive(Debug, Deserialize)]
pub struct PageParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>
}


pub struct Pagination {
    pub page: u32, 
    pub per_page: u32,
    pub offset: i64
}


impl TryFrom<PageParams> for Pagination {
    type Error = ApiError;
    fn try_from(raw: PageParams) -> Result<Self, Self::Error> {
        let page = raw.page.unwrap_or(1);
        let per_page = raw.per_page.unwrap_or(10);

        let mut field_errors = Vec::new();

        if page == 0 {
            field_errors.push(FieldError{field: "page".into(), message: "must be >=1".into()});
        }
        if per_page == 0 || per_page >100 {
            field_errors.push(FieldError{field: "per_page".into(), message: "must be >=1 and <=10".into()});
        }
        if !field_errors.is_empty() {
            return Err(
                ApiError::Validation {
                    message: "Invalid pagination parameters", 
                    errors: field_errors
                }
            );
        }

        Ok(Pagination {
            page, 
            per_page, 
            offset: ((page -1) as i64)*(per_page as i64)
        })
    }
}
