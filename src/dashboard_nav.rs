#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DashboardPage {
    Investments,
    Investors,
    Startups,
    Review,
}

impl DashboardPage {
    pub fn title(self) -> &'static str {
        match self {
            Self::Investments => "Dashboard",
            Self::Investors => "Investors",
            Self::Startups => "Startups",
            Self::Review => "Review",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DashboardPage;

    #[test]
    fn startups_has_independent_page_title() {
        assert_eq!(DashboardPage::Startups.title(), "Startups");
    }

    #[test]
    fn investors_has_independent_page_title() {
        assert_eq!(DashboardPage::Investors.title(), "Investors");
    }

    #[test]
    fn review_has_independent_page_title() {
        assert_eq!(DashboardPage::Review.title(), "Review");
    }
}
