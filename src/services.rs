use crate::models::UserData;

#[derive(Debug)]
pub enum ComplianceError {
    Underage,
    PendingDocuments,
}

pub fn check_compliance(user: &UserData) -> Result<String, ComplianceError> {
    if user.age < 18 {
        return Err(ComplianceError::Underage);
    }

    if user.document_status != "VERIFIED" {
        return Err(ComplianceError::PendingDocuments);
    }

    Ok(format!("User {} is compliant.", user.user_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_underage_is_rejected() {
        let user = UserData {
            user_id: "U-TEST-1".to_string(),
            age: 17,
            document_status: "VERIFIED".to_string(),
        };

        let result = check_compliance(&user);
        assert!(matches!(result, Err(ComplianceError::Underage)));
    }

    #[test]
    fn test_pending_document_is_rejected() {
        let user = UserData {
            user_id: "U-TEST-2".to_string(),
            age: 22,
            document_status: "PENDING".to_string(),
        };

        let result = check_compliance(&user);
        assert!(matches!(result, Err(ComplianceError::PendingDocuments)));
    }

    #[test]
    fn test_valid_user_is_approved() {
        let user = UserData {
            user_id: "U-TEST-3".to_string(),
            age: 25,
            document_status: "VERIFIED".to_string(),
        };

        let result = check_compliance(&user);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "User U-TEST-3 is compliant.");
    }
}
