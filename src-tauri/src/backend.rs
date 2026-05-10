use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CURRENT_NDA_VERSION: &str = "SABER_INVESTOR_NDA_V1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Investment {
    pub id: String,
    pub business_id: String,
    pub company: String,
    pub contact: String,
    pub email: String,
    pub stage: String,
    pub capital: u64,
    pub story: String,
    pub duns_number: String,
    pub licensing: String,
    pub business_plan: String,
    pub pictures: Vec<String>,
    pub status: ReviewStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Investor {
    pub id: String,
    pub name: String,
    pub email: String,
    pub accredited: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct InvestorRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
    pub accredited: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SmallBusinessAccount {
    pub id: String,
    pub company: String,
    pub contact: String,
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SmallBusinessRegistration {
    pub company: String,
    pub contact: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AccountCredential {
    id: String,
    email: String,
    password: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NdaSignature {
    pub investor_id: String,
    pub startup_id: String,
    pub legal_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub initials: String,
    pub nda_version: String,
    pub signed_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConnectionRequest {
    pub id: String,
    pub investor_id: String,
    pub startup_id: String,
    pub business_id: String,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum BusinessPlanRequestStatus {
    Pending,
    Approved,
    Declined,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BusinessPlanRequest {
    pub id: String,
    pub investor_id: String,
    pub investor_name: String,
    pub startup_id: String,
    pub business_id: String,
    pub company: String,
    pub status: BusinessPlanRequestStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum FundingStatus {
    PendingReview,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FundingIntent {
    pub id: String,
    pub investor_id: String,
    pub startup_id: String,
    pub amount_cents: u64,
    pub status: FundingStatus,
    pub stripe_checkout_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct StartupSubmission {
    pub business_id: String,
    pub company: String,
    pub contact: String,
    pub email: String,
    pub stage: String,
    pub capital: u64,
    pub story: String,
    pub duns_number: String,
    pub licensing: String,
    pub business_plan: String,
    pub pictures: Vec<String>,
}

#[derive(Default)]
pub struct AppStore {
    investments: Mutex<Vec<Investment>>,
    investors: Mutex<Vec<Investor>>,
    investor_credentials: Mutex<Vec<AccountCredential>>,
    businesses: Mutex<Vec<SmallBusinessAccount>>,
    business_credentials: Mutex<Vec<AccountCredential>>,
    nda_signatures: Mutex<Vec<NdaSignature>>,
    business_plan_requests: Mutex<Vec<BusinessPlanRequest>>,
    connection_requests: Mutex<Vec<ConnectionRequest>>,
    funding_intents: Mutex<Vec<FundingIntent>>,
}

impl AppStore {
    pub fn list_investments(&self) -> Vec<Investment> {
        self.investments
            .lock()
            .unwrap()
            .iter()
            .filter(|investment| investment.status == ReviewStatus::Approved)
            .cloned()
            .collect()
    }

    pub fn list_startups(&self) -> Vec<Investment> {
        self.investments.lock().unwrap().clone()
    }

    pub fn submit_startup(&self, submission: StartupSubmission) -> Investment {
        let mut investments = self.investments.lock().unwrap();
        let investment = Investment {
            id: format!("startup-{}", investments.len() + 1),
            business_id: submission.business_id.trim().to_string(),
            company: submission.company.trim().to_string(),
            contact: submission.contact.trim().to_string(),
            email: submission.email.trim().to_string(),
            stage: submission.stage.trim().to_string(),
            capital: submission.capital,
            story: submission.story.trim().to_string(),
            duns_number: submission.duns_number.trim().to_string(),
            licensing: submission.licensing.trim().to_string(),
            business_plan: submission.business_plan.trim().to_string(),
            pictures: submission
                .pictures
                .into_iter()
                .map(|picture| picture.trim().to_string())
                .filter(|picture| !picture.is_empty())
                .collect(),
            status: ReviewStatus::Pending,
        };

        investments.push(investment.clone());
        investment
    }

    pub fn approve_startup(&self, id: &str) -> Result<Investment, String> {
        self.set_review_status(id, ReviewStatus::Approved)
    }

    pub fn reject_startup(&self, id: &str) -> Result<Investment, String> {
        self.set_review_status(id, ReviewStatus::Rejected)
    }

    fn set_review_status(&self, id: &str, status: ReviewStatus) -> Result<Investment, String> {
        let mut investments = self.investments.lock().unwrap();
        let Some(investment) = investments
            .iter_mut()
            .find(|investment| investment.id == id)
        else {
            return Err("Startup not found".to_string());
        };

        investment.status = status;
        Ok(investment.clone())
    }

    pub fn register_investor(&self, registration: InvestorRegistration) -> Result<Investor, String> {
        let email = registration.email.trim().to_lowercase();
        if email.is_empty() || registration.password.trim().is_empty() {
            return Err("Investor credentials required".to_string());
        }
        if self
            .investor_credentials
            .lock()
            .unwrap()
            .iter()
            .any(|credential| credential.email == email)
        {
            return Err("Investor already exists".to_string());
        }

        let mut investors = self.investors.lock().unwrap();
        let investor = Investor {
            id: format!("investor-{}", investors.len() + 1),
            name: registration.name.trim().to_string(),
            email: email.clone(),
            accredited: registration.accredited,
        };

        investors.push(investor.clone());
        self.investor_credentials
            .lock()
            .unwrap()
            .push(AccountCredential {
                id: investor.id.clone(),
                email,
                password: registration.password,
            });

        Ok(investor)
    }

    pub fn sign_in_investor(&self, email: &str, password: &str) -> Result<Investor, String> {
        let email = email.trim().to_lowercase();
        let password = password.trim();
        if is_developer_login(&email, password) {
            return Ok(Investor {
                id: "developer-investor".to_string(),
                name: "Developer Investor".to_string(),
                email,
                accredited: false,
            });
        }

        let Some(credential) = self
            .investor_credentials
            .lock()
            .unwrap()
            .iter()
            .find(|credential| credential.email == email && credential.password == password)
            .cloned()
        else {
            return Err("Investor sign in denied".to_string());
        };

        self.require_investor(&credential.id)
    }

    pub fn register_business(
        &self,
        registration: SmallBusinessRegistration,
    ) -> Result<SmallBusinessAccount, String> {
        let email = registration.email.trim().to_lowercase();
        if email.is_empty() || registration.password.trim().is_empty() {
            return Err("Business credentials required".to_string());
        }
        if self
            .business_credentials
            .lock()
            .unwrap()
            .iter()
            .any(|credential| credential.email == email)
        {
            return Err("Business already exists".to_string());
        }

        let mut businesses = self.businesses.lock().unwrap();
        let business = SmallBusinessAccount {
            id: format!("business-{}", businesses.len() + 1),
            company: registration.company.trim().to_string(),
            contact: registration.contact.trim().to_string(),
            email: email.clone(),
        };

        businesses.push(business.clone());
        self.business_credentials
            .lock()
            .unwrap()
            .push(AccountCredential {
                id: business.id.clone(),
                email,
                password: registration.password,
            });

        Ok(business)
    }

    pub fn sign_in_business(
        &self,
        email: &str,
        password: &str,
    ) -> Result<SmallBusinessAccount, String> {
        let email = email.trim().to_lowercase();
        let password = password.trim();
        if is_developer_login(&email, password) {
            return Ok(SmallBusinessAccount {
                id: "developer-business".to_string(),
                company: "Developer Business".to_string(),
                contact: "Developer".to_string(),
                email,
            });
        }

        let Some(credential) = self
            .business_credentials
            .lock()
            .unwrap()
            .iter()
            .find(|credential| credential.email == email && credential.password == password)
            .cloned()
        else {
            return Err("Business sign in denied".to_string());
        };

        self.businesses
            .lock()
            .unwrap()
            .iter()
            .find(|business| business.id == credential.id)
            .cloned()
            .ok_or_else(|| "Business not found".to_string())
    }

    pub fn sign_nda(
        &self,
        investor_id: &str,
        startup_id: &str,
        legal_name: &str,
        email: &str,
        phone: &str,
        address: &str,
        initials: &str,
    ) -> Result<NdaSignature, String> {
        self.require_investor(investor_id)?;
        self.require_approved_startup(startup_id)?;
        if legal_name.trim().is_empty()
            || email.trim().is_empty()
            || phone.trim().is_empty()
            || address.trim().is_empty()
            || initials.trim().is_empty()
        {
            return Err("Complete NDA signature required".to_string());
        }

        let mut signatures = self.nda_signatures.lock().unwrap();
        let signed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs().to_string())
            .unwrap_or_else(|_| "0".to_string());
        let signature = NdaSignature {
            investor_id: investor_id.to_string(),
            startup_id: startup_id.to_string(),
            legal_name: legal_name.trim().to_string(),
            email: email.trim().to_string(),
            phone: phone.trim().to_string(),
            address: address.trim().to_string(),
            initials: initials.trim().to_string(),
            nda_version: CURRENT_NDA_VERSION.to_string(),
            signed_at,
        };

        signatures.push(signature.clone());
        Ok(signature)
    }

    pub fn business_plan_for_investor(
        &self,
        investor_id: &str,
        startup_id: &str,
    ) -> Result<String, String> {
        if !self.has_signed_nda(investor_id, startup_id) {
            return Err("NDA required".to_string());
        }
        if !self.has_approved_business_plan_request(investor_id, startup_id) {
            return Err("Business approval required".to_string());
        }

        Ok(self.require_approved_startup(startup_id)?.business_plan)
    }

    pub fn request_business_plan(
        &self,
        investor_id: &str,
        startup_id: &str,
    ) -> Result<BusinessPlanRequest, String> {
        let investor = self.require_investor(investor_id)?;
        let startup = self.require_approved_startup(startup_id)?;
        if !self.has_signed_nda(investor_id, startup_id) {
            return Err("NDA required".to_string());
        }

        let mut requests = self.business_plan_requests.lock().unwrap();
        if let Some(existing) = requests
            .iter()
            .find(|request| request.investor_id == investor_id && request.startup_id == startup_id)
            .cloned()
        {
            return Ok(existing);
        }

        let request = BusinessPlanRequest {
            id: format!("plan-request-{}", requests.len() + 1),
            investor_id: investor_id.to_string(),
            investor_name: investor.name,
            startup_id: startup_id.to_string(),
            business_id: startup.business_id,
            company: startup.company,
            status: BusinessPlanRequestStatus::Pending,
        };
        requests.push(request.clone());
        Ok(request)
    }

    pub fn list_business_plan_requests_for_business(
        &self,
        business_id: &str,
    ) -> Vec<BusinessPlanRequest> {
        self.business_plan_requests
            .lock()
            .unwrap()
            .iter()
            .filter(|request| request.business_id == business_id)
            .cloned()
            .collect()
    }

    pub fn approve_business_plan_request(
        &self,
        business_id: &str,
        request_id: &str,
    ) -> Result<BusinessPlanRequest, String> {
        self.set_business_plan_request_status(
            business_id,
            request_id,
            BusinessPlanRequestStatus::Approved,
        )
    }

    pub fn decline_business_plan_request(
        &self,
        business_id: &str,
        request_id: &str,
    ) -> Result<BusinessPlanRequest, String> {
        self.set_business_plan_request_status(
            business_id,
            request_id,
            BusinessPlanRequestStatus::Declined,
        )
    }

    pub fn request_connection(
        &self,
        investor_id: &str,
        startup_id: &str,
        business_id: &str,
        message: &str,
    ) -> Result<ConnectionRequest, String> {
        if !self.has_signed_nda(investor_id, startup_id) {
            return Err("NDA required".to_string());
        }
        let startup = self.require_approved_startup(startup_id)?;
        if startup.business_id != business_id.trim() {
            return Err("Business ID mismatch".to_string());
        }

        let mut requests = self.connection_requests.lock().unwrap();
        let request = ConnectionRequest {
            id: format!("connection-{}", requests.len() + 1),
            investor_id: investor_id.to_string(),
            startup_id: startup_id.to_string(),
            business_id: business_id.trim().to_string(),
            message: message.trim().to_string(),
        };

        requests.push(request.clone());
        Ok(request)
    }

    pub fn create_funding_intent(
        &self,
        investor_id: &str,
        startup_id: &str,
        amount_cents: u64,
    ) -> Result<FundingIntent, String> {
        if amount_cents == 0 {
            return Err("Amount required".to_string());
        }
        if !self.has_signed_nda(investor_id, startup_id) {
            return Err("NDA required".to_string());
        }

        let mut intents = self.funding_intents.lock().unwrap();
        let intent = FundingIntent {
            id: format!("funding-{}", intents.len() + 1),
            investor_id: investor_id.to_string(),
            startup_id: startup_id.to_string(),
            amount_cents,
            status: FundingStatus::PendingReview,
            stripe_checkout_url: None,
        };

        intents.push(intent.clone());
        Ok(intent)
    }

    fn has_signed_nda(&self, investor_id: &str, startup_id: &str) -> bool {
        self.nda_signatures
            .lock()
            .unwrap()
            .iter()
            .any(|signature| {
                signature.investor_id == investor_id && signature.startup_id == startup_id
            })
    }

    fn has_approved_business_plan_request(&self, investor_id: &str, startup_id: &str) -> bool {
        self.business_plan_requests
            .lock()
            .unwrap()
            .iter()
            .any(|request| {
                request.investor_id == investor_id
                    && request.startup_id == startup_id
                    && request.status == BusinessPlanRequestStatus::Approved
            })
    }

    fn set_business_plan_request_status(
        &self,
        business_id: &str,
        request_id: &str,
        status: BusinessPlanRequestStatus,
    ) -> Result<BusinessPlanRequest, String> {
        let mut requests = self.business_plan_requests.lock().unwrap();
        let request = requests
            .iter_mut()
            .find(|request| request.id == request_id && request.business_id == business_id)
            .ok_or_else(|| "Business plan request not found".to_string())?;
        request.status = status;
        Ok(request.clone())
    }

    fn require_investor(&self, investor_id: &str) -> Result<Investor, String> {
        self.investors
            .lock()
            .unwrap()
            .iter()
            .find(|investor| investor.id == investor_id)
            .cloned()
            .ok_or_else(|| "Investor not found".to_string())
    }

    fn require_approved_startup(&self, startup_id: &str) -> Result<Investment, String> {
        self.investments
            .lock()
            .unwrap()
            .iter()
            .find(|startup| {
                startup.id == startup_id && startup.status == ReviewStatus::Approved
            })
            .cloned()
            .ok_or_else(|| "Approved startup not found".to_string())
    }
}

pub fn authenticate_admin(username: &str, password: &str) -> bool {
    username.trim() == "admin" && password.trim() == "admin"
}

fn is_developer_login(username: &str, password: &str) -> bool {
    username.trim().eq_ignore_ascii_case("developer") && password.trim() == "developer"
}

#[tauri::command]
pub fn admin_sign_in(username: String, password: String) -> bool {
    authenticate_admin(&username, &password)
}

#[tauri::command]
pub fn get_investments(store: tauri::State<AppStore>) -> Vec<Investment> {
    store.list_investments()
}

#[tauri::command]
pub fn get_startups(store: tauri::State<AppStore>) -> Vec<Investment> {
    store.list_startups()
}

#[tauri::command]
pub fn submit_startup(
    store: tauri::State<AppStore>,
    submission: StartupSubmission,
) -> Investment {
    store.submit_startup(submission)
}

#[tauri::command]
pub fn approve_startup(store: tauri::State<AppStore>, id: String) -> Result<Investment, String> {
    store.approve_startup(&id)
}

#[tauri::command]
pub fn reject_startup(store: tauri::State<AppStore>, id: String) -> Result<Investment, String> {
    store.reject_startup(&id)
}

#[tauri::command]
pub fn register_investor(
    store: tauri::State<AppStore>,
    registration: InvestorRegistration,
) -> Result<Investor, String> {
    store.register_investor(registration)
}

#[tauri::command]
pub fn investor_sign_in(
    store: tauri::State<AppStore>,
    email: String,
    password: String,
) -> Result<Investor, String> {
    store.sign_in_investor(&email, &password)
}

#[tauri::command]
pub fn register_business(
    store: tauri::State<AppStore>,
    registration: SmallBusinessRegistration,
) -> Result<SmallBusinessAccount, String> {
    store.register_business(registration)
}

#[tauri::command]
pub fn business_sign_in(
    store: tauri::State<AppStore>,
    email: String,
    password: String,
) -> Result<SmallBusinessAccount, String> {
    store.sign_in_business(&email, &password)
}

#[tauri::command]
pub fn sign_nda(
    store: tauri::State<AppStore>,
    investor_id: String,
    startup_id: String,
    legal_name: String,
    email: String,
    phone: String,
    address: String,
    initials: String,
) -> Result<NdaSignature, String> {
    store.sign_nda(
        &investor_id,
        &startup_id,
        &legal_name,
        &email,
        &phone,
        &address,
        &initials,
    )
}

#[tauri::command]
pub fn business_plan_for_investor(
    store: tauri::State<AppStore>,
    investor_id: String,
    startup_id: String,
) -> Result<String, String> {
    store.business_plan_for_investor(&investor_id, &startup_id)
}

#[tauri::command]
pub fn request_business_plan(
    store: tauri::State<AppStore>,
    investor_id: String,
    startup_id: String,
) -> Result<BusinessPlanRequest, String> {
    store.request_business_plan(&investor_id, &startup_id)
}

#[tauri::command]
pub fn list_business_plan_requests_for_business(
    store: tauri::State<AppStore>,
    business_id: String,
) -> Vec<BusinessPlanRequest> {
    store.list_business_plan_requests_for_business(&business_id)
}

#[tauri::command]
pub fn approve_business_plan_request(
    store: tauri::State<AppStore>,
    business_id: String,
    request_id: String,
) -> Result<BusinessPlanRequest, String> {
    store.approve_business_plan_request(&business_id, &request_id)
}

#[tauri::command]
pub fn decline_business_plan_request(
    store: tauri::State<AppStore>,
    business_id: String,
    request_id: String,
) -> Result<BusinessPlanRequest, String> {
    store.decline_business_plan_request(&business_id, &request_id)
}

#[tauri::command]
pub fn request_connection(
    store: tauri::State<AppStore>,
    investor_id: String,
    startup_id: String,
    business_id: String,
    message: String,
) -> Result<ConnectionRequest, String> {
    store.request_connection(&investor_id, &startup_id, &business_id, &message)
}

#[tauri::command]
pub fn create_funding_intent(
    store: tauri::State<AppStore>,
    investor_id: String,
    startup_id: String,
    amount_cents: u64,
) -> Result<FundingIntent, String> {
    store.create_funding_intent(&investor_id, &startup_id, amount_cents)
}

#[cfg(test)]
mod tests {
    use super::{
        authenticate_admin, AppStore, BusinessPlanRequestStatus, FundingStatus,
        InvestorRegistration, ReviewStatus, SmallBusinessRegistration, StartupSubmission,
    };

    fn sign_test_nda(store: &AppStore, investor_id: &str, startup_id: &str) {
        store
            .sign_nda(
                investor_id,
                startup_id,
                "Small Investor",
                "investor@example.com",
                "555-0100",
                "100 Capital Way",
                "SI",
            )
            .unwrap();
    }

    #[test]
    fn authenticates_admin_credentials() {
        assert!(authenticate_admin("admin", "admin"));
    }

    #[test]
    fn rejects_non_admin_credentials() {
        assert!(!authenticate_admin("admin", "wrong"));
    }

    #[test]
    fn starts_with_no_investments() {
        assert!(AppStore::default().list_investments().is_empty());
    }

    #[test]
    fn lists_submitted_startup() {
        let store = AppStore::default();

        store.submit_startup(StartupSubmission {
            business_id: "business-1".to_string(),
            company: "Foundry Labs".to_string(),
            contact: "Avery Stone".to_string(),
            email: "avery@foundry.test".to_string(),
            stage: "Seed".to_string(),
            capital: 250000,
            story: "Building private infrastructure tools.".to_string(),
            duns_number: "123456789".to_string(),
            licensing: "Delaware registration active".to_string(),
            business_plan: "foundry-plan.pdf".to_string(),
            pictures: vec!["foundry-lab.png".to_string()],
        });

        let startups = store.list_startups();
        assert_eq!(startups.len(), 1);
        assert_eq!(startups[0].company, "Foundry Labs");
        assert_eq!(startups[0].capital, 250000);
    }

    #[test]
    fn submitted_startup_starts_pending_with_private_fields() {
        let store = AppStore::default();

        let startup = store.submit_startup(StartupSubmission {
            business_id: "business-1".to_string(),
            company: "Northstar Robotics".to_string(),
            contact: "Mara Vale".to_string(),
            email: "mara@northstar.test".to_string(),
            stage: "Prototype".to_string(),
            capital: 750000,
            story: "Autonomous inspection for private industrial sites.".to_string(),
            duns_number: "987654321".to_string(),
            licensing: "State robotics permit pending".to_string(),
            business_plan: "northstar-plan.pdf".to_string(),
            pictures: vec!["robot-cell.png".to_string(), "field-unit.png".to_string()],
        });

        assert_eq!(startup.status, ReviewStatus::Pending);
        assert_eq!(startup.duns_number, "987654321");
        assert_eq!(startup.licensing, "State robotics permit pending");
        assert_eq!(startup.business_plan, "northstar-plan.pdf");
        assert_eq!(startup.pictures.len(), 2);
    }

    #[test]
    fn only_approved_startups_are_listed_for_investment() {
        let store = AppStore::default();

        let pending = store.submit_startup(StartupSubmission {
            business_id: "business-1".to_string(),
            company: "Pending Co".to_string(),
            contact: "Pat Pending".to_string(),
            email: "pat@pending.test".to_string(),
            stage: "Seed".to_string(),
            capital: 100000,
            story: "Pending review.".to_string(),
            duns_number: "111111111".to_string(),
            licensing: "Active".to_string(),
            business_plan: "pending.pdf".to_string(),
            pictures: Vec::new(),
        });
        let approved = store.submit_startup(StartupSubmission {
            business_id: "business-2".to_string(),
            company: "Approved Co".to_string(),
            contact: "Ari Approved".to_string(),
            email: "ari@approved.test".to_string(),
            stage: "Series A".to_string(),
            capital: 900000,
            story: "Approved private deal.".to_string(),
            duns_number: "222222222".to_string(),
            licensing: "Active".to_string(),
            business_plan: "approved.pdf".to_string(),
            pictures: Vec::new(),
        });
        let rejected = store.submit_startup(StartupSubmission {
            business_id: "business-3".to_string(),
            company: "Rejected Co".to_string(),
            contact: "Rae Rejected".to_string(),
            email: "rae@rejected.test".to_string(),
            stage: "Concept".to_string(),
            capital: 50000,
            story: "Rejected review.".to_string(),
            duns_number: "333333333".to_string(),
            licensing: "Inactive".to_string(),
            business_plan: "rejected.pdf".to_string(),
            pictures: Vec::new(),
        });

        store.approve_startup(&approved.id).unwrap();
        store.reject_startup(&rejected.id).unwrap();

        let listed = store.list_investments();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, approved.id);
        assert_ne!(listed[0].id, pending.id);
        assert_ne!(listed[0].id, rejected.id);
    }

    #[test]
    fn investor_must_sign_nda_before_business_plan_access() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();

        assert!(store
            .business_plan_for_investor(&investor.id, &startup.id)
            .is_err());

        sign_test_nda(&store, &investor.id, &startup.id);

        let request = store
            .request_business_plan(&investor.id, &startup.id)
            .unwrap();
        store
            .approve_business_plan_request(&startup.business_id, &request.id)
            .unwrap();

        assert_eq!(
            store
                .business_plan_for_investor(&investor.id, &startup.id)
                .unwrap(),
            "approved.pdf"
        );
    }

    #[test]
    fn investor_must_get_business_approval_before_business_plan_access() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();
        sign_test_nda(&store, &investor.id, &startup.id);

        assert!(store
            .business_plan_for_investor(&investor.id, &startup.id)
            .is_err());

        let request = store
            .request_business_plan(&investor.id, &startup.id)
            .unwrap();
        assert_eq!(request.status, BusinessPlanRequestStatus::Pending);
        assert!(store
            .business_plan_for_investor(&investor.id, &startup.id)
            .is_err());

        store
            .approve_business_plan_request(&startup.business_id, &request.id)
            .unwrap();

        assert_eq!(
            store
                .business_plan_for_investor(&investor.id, &startup.id)
                .unwrap(),
            "approved.pdf"
        );
    }

    #[test]
    fn business_can_decline_business_plan_request() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();
        sign_test_nda(&store, &investor.id, &startup.id);

        let request = store
            .request_business_plan(&investor.id, &startup.id)
            .unwrap();
        let listed = store.list_business_plan_requests_for_business(&startup.business_id);
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].investor_name, "Small Investor");

        let declined = store
            .decline_business_plan_request(&startup.business_id, &request.id)
            .unwrap();

        assert_eq!(declined.status, BusinessPlanRequestStatus::Declined);
        assert!(store
            .business_plan_for_investor(&investor.id, &startup.id)
            .is_err());
    }

    #[test]
    fn small_investor_can_create_funding_intent_after_nda() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();
        sign_test_nda(&store, &investor.id, &startup.id);

        let intent = store
            .create_funding_intent(&investor.id, &startup.id, 2500)
            .unwrap();

        assert_eq!(intent.amount_cents, 2500);
        assert_eq!(intent.status, FundingStatus::PendingReview);
        assert!(intent.stripe_checkout_url.is_none());
    }

    #[test]
    fn nda_signature_records_current_agreement_version() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();

        let signature = store
            .sign_nda(
                &investor.id,
                &startup.id,
                "Small Investor",
                "investor@example.com",
                "555-0100",
                "100 Capital Way",
                "SI",
            )
            .unwrap();

        assert_eq!(signature.nda_version, "SABER_INVESTOR_NDA_V1");
        assert_eq!(signature.email, "investor@example.com");
        assert_eq!(signature.phone, "555-0100");
        assert_eq!(signature.address, "100 Capital Way");
        assert_eq!(signature.initials, "SI");
        assert!(!signature.signed_at.is_empty());
    }

    #[test]
    fn nda_signature_requires_complete_signing_block() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();

        assert!(store
            .sign_nda(
                &investor.id,
                &startup.id,
                "Small Investor",
                "investor@example.com",
                "",
                "100 Capital Way",
                "SI",
            )
            .is_err());
    }

    #[test]
    fn unsigned_investor_cannot_request_connection_or_funding() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store.register_investor(InvestorRegistration {
            name: "Small Investor".to_string(),
            email: "investor@example.com".to_string(),
            password: "investor".to_string(),
            accredited: false,
        }).unwrap();

        assert!(store
            .request_connection(&investor.id, &startup.id, &startup.business_id, "Interested")
            .is_err());
        assert!(store
            .create_funding_intent(&investor.id, &startup.id, 1000)
            .is_err());
    }

    #[test]
    fn connection_requires_matching_business_id_after_nda() {
        let store = AppStore::default();
        let startup = approved_startup(&store);
        let investor = store
            .register_investor(InvestorRegistration {
                name: "Small Investor".to_string(),
                email: "investor@example.com".to_string(),
                password: "investor".to_string(),
                accredited: false,
            })
            .unwrap();
        sign_test_nda(&store, &investor.id, &startup.id);

        assert!(store
            .request_connection(&investor.id, &startup.id, "wrong-business", "Interested")
            .is_err());

        let request = store
            .request_connection(&investor.id, &startup.id, &startup.business_id, "Interested")
            .unwrap();
        assert_eq!(request.business_id, startup.business_id);
    }

    #[test]
    fn investor_can_sign_up_and_sign_in() {
        let store = AppStore::default();

        let registered = store
            .register_investor(InvestorRegistration {
                name: "Avery Investor".to_string(),
                email: "avery@investor.test".to_string(),
                password: "private".to_string(),
                accredited: false,
            })
            .unwrap();
        let signed_in = store
            .sign_in_investor("avery@investor.test", "private")
            .unwrap();

        assert_eq!(signed_in, registered);
        assert!(store.sign_in_investor("avery@investor.test", "wrong").is_err());
    }

    #[test]
    fn business_can_sign_up_and_sign_in() {
        let store = AppStore::default();

        let registered = store
            .register_business(SmallBusinessRegistration {
                company: "Foundry Labs".to_string(),
                contact: "Avery Stone".to_string(),
                email: "avery@foundry.test".to_string(),
                password: "private".to_string(),
            })
            .unwrap();
        let signed_in = store
            .sign_in_business("avery@foundry.test", "private")
            .unwrap();

        assert_eq!(signed_in, registered);
        assert!(store.sign_in_business("avery@foundry.test", "wrong").is_err());
    }

    #[test]
    fn developer_can_access_investor_portal() {
        let signed_in = AppStore::default()
            .sign_in_investor("developer", "developer")
            .unwrap();

        assert_eq!(signed_in.id, "developer-investor");
        assert_eq!(signed_in.name, "Developer Investor");
    }

    #[test]
    fn developer_can_access_business_portal() {
        let signed_in = AppStore::default()
            .sign_in_business("developer", "developer")
            .unwrap();

        assert_eq!(signed_in.id, "developer-business");
        assert_eq!(signed_in.company, "Developer Business");
    }

    fn approved_startup(store: &AppStore) -> super::Investment {
        let startup = store.submit_startup(StartupSubmission {
            business_id: "business-2".to_string(),
            company: "Approved Co".to_string(),
            contact: "Ari Approved".to_string(),
            email: "ari@approved.test".to_string(),
            stage: "Series A".to_string(),
            capital: 900000,
            story: "Approved private deal.".to_string(),
            duns_number: "222222222".to_string(),
            licensing: "Active".to_string(),
            business_plan: "approved.pdf".to_string(),
            pictures: Vec::new(),
        });
        store.approve_startup(&startup.id).unwrap()
    }
}
