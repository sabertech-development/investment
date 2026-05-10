pub fn nda_form_complete(
    legal_name: &str,
    email: &str,
    phone: &str,
    address: &str,
    initials: &str,
    accepted: bool,
) -> bool {
    accepted
        && !legal_name.trim().is_empty()
        && !email.trim().is_empty()
        && !phone.trim().is_empty()
        && !address.trim().is_empty()
        && !initials.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::nda_form_complete;

    #[test]
    fn complete_nda_form_unlocks_signing() {
        assert!(nda_form_complete(
            "Morgan Investor",
            "morgan@example.com",
            "555-0100",
            "100 Capital Way",
            "MI",
            true,
        ));
    }

    #[test]
    fn incomplete_nda_form_stays_locked() {
        assert!(!nda_form_complete(
            "Morgan Investor",
            "morgan@example.com",
            "",
            "100 Capital Way",
            "MI",
            true,
        ));
        assert!(!nda_form_complete(
            "Morgan Investor",
            "morgan@example.com",
            "555-0100",
            "100 Capital Way",
            "MI",
            false,
        ));
    }
}
