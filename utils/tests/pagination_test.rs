#[cfg(test)]
mod tests {
    use utils::pagination::*;
    use utils::http_error::ApiError;

    #[test]
    fn valid_pagination_defaults() {
        let params = PageParams {
            page: None, 
            per_page: None
        };
        let pagination = Pagination::try_from(params).unwrap();
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.per_page, 10);
        assert_eq!(pagination.offset, 0);
    }

    #[test]
    fn valid_pagination_custom_values() {
        let params = PageParams { page: Some(2), per_page: Some(20) };
        let pagination = Pagination::try_from(params).unwrap();
        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.per_page, 20);
        assert_eq!(pagination.offset, 20);
    }
    
    #[test]
    fn invalid_page_zero() {
        let params = PageParams { page: Some(0), per_page: Some(10) };
        let result = Pagination::try_from(params);
        assert!(matches!(result, Err(ApiError::Validation { .. })));
    }
    

    #[test]
    fn invalid_per_page_too_large() {
        let params = PageParams { page: Some(1), per_page: Some(200) };
        let result = Pagination::try_from(params);
        assert!(matches!(result, Err(ApiError::Validation { .. })));
    }

    #[test]
    fn invalid_both_fields() {
        let params = PageParams { page: Some(0), per_page: Some(0) };
        let result = Pagination::try_from(params);
        if let Err(ApiError::Validation { message, errors }) = result {
            assert_eq!(message, "Invalid pagination parameters");
            assert_eq!(errors.len(), 2);
            assert_eq!(errors[0].field, "page");
            assert_eq!(errors[1].field, "per_page");
        } else {
            panic!("Expected validation error");
        }
    }    
}