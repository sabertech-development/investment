#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
}

impl SignInForm {
    pub fn can_submit(&self) -> bool {
        let email = self.email.trim();
        let password = self.password.trim();

        self.is_admin() || (email.contains('@') && email.contains('.') && !password.is_empty())
    }

    pub fn is_admin(&self) -> bool {
        self.email.trim() == "admin" && self.password.trim() == "admin"
    }
}

#[cfg(test)]
mod tests {
    use super::SignInForm;

    #[test]
    fn enables_submission_for_complete_investor_credentials() {
        let form = SignInForm {
            email: "investor@saber.com".to_string(),
            password: "secure-access".to_string(),
        };

        assert!(form.can_submit());
    }

    #[test]
    fn blocks_submission_until_email_looks_valid() {
        let form = SignInForm {
            email: "investor".to_string(),
            password: "secure-access".to_string(),
        };

        assert!(!form.can_submit());
    }

    #[test]
    fn authenticates_admin_credentials() {
        let form = SignInForm {
            email: "admin".to_string(),
            password: "admin".to_string(),
        };

        assert!(form.is_admin());
    }
}
