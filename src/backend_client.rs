use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub enum ReviewStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Investor {
    pub id: String,
    pub name: String,
    pub email: String,
    pub accredited: bool,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct InvestorRegistration {
    pub name: String,
    pub email: String,
    pub password: String,
    pub accredited: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SmallBusinessAccount {
    pub id: String,
    pub company: String,
    pub contact: String,
    pub email: String,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SmallBusinessRegistration {
    pub company: String,
    pub contact: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct ConnectionRequest {
    pub id: String,
    pub investor_id: String,
    pub startup_id: String,
    pub business_id: String,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub enum BusinessPlanRequestStatus {
    #[default]
    Pending,
    Approved,
    Declined,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BusinessPlanRequest {
    pub id: String,
    pub investor_id: String,
    pub investor_name: String,
    pub startup_id: String,
    pub business_id: String,
    pub company: String,
    pub status: BusinessPlanRequestStatus,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub enum FundingStatus {
    #[default]
    PendingReview,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct FundingIntent {
    pub id: String,
    pub investor_id: String,
    pub startup_id: String,
    pub amount_cents: u64,
    pub status: FundingStatus,
    pub stripe_checkout_url: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
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

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct SignInArgs {
    username: String,
    password: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct SubmitStartupArgs {
    submission: StartupSubmission,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct InvestorRegistrationArgs {
    registration: InvestorRegistration,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct AccountSignInArgs {
    email: String,
    password: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct SmallBusinessRegistrationArgs {
    registration: SmallBusinessRegistration,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct SignNdaArgs {
    investor_id: String,
    startup_id: String,
    legal_name: String,
    email: String,
    phone: String,
    address: String,
    initials: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct BusinessPlanArgs {
    investor_id: String,
    startup_id: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct BusinessPlanRequestsForBusinessArgs {
    business_id: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct BusinessPlanRequestDecisionArgs {
    business_id: String,
    request_id: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct RequestConnectionArgs {
    investor_id: String,
    startup_id: String,
    business_id: String,
    message: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct CreateFundingIntentArgs {
    investor_id: String,
    startup_id: String,
    amount_cents: u64,
}

#[cfg(target_arch = "wasm32")]
#[derive(serde::Serialize)]
struct ReviewStartupArgs {
    id: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"],
        js_name = invoke,
        catch
    )]
    async fn tauri_invoke(
        command: &str,
        args: wasm_bindgen::JsValue,
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;
}

#[cfg(target_arch = "wasm32")]
fn tauri_available() -> bool {
    use wasm_bindgen::JsValue;

    let global = js_sys::global();
    let Ok(tauri) = js_sys::Reflect::get(&global, &JsValue::from_str("__TAURI__")) else {
        return false;
    };
    let Ok(core) = js_sys::Reflect::get(&tauri, &JsValue::from_str("core")) else {
        return false;
    };
    let Ok(invoke) = js_sys::Reflect::get(&core, &JsValue::from_str("invoke")) else {
        return false;
    };

    invoke.is_function()
}

fn local_admin_sign_in(username: &str, password: &str) -> bool {
    username.trim() == "admin" && password.trim() == "admin"
}

#[cfg(target_arch = "wasm32")]
pub async fn admin_sign_in(username: String, password: String) -> bool {
    if !tauri_available() {
        return local_admin_sign_in(&username, &password);
    }

    let args = serde_wasm_bindgen::to_value(&SignInArgs { username, password });
    let Ok(args) = args else {
        return false;
    };

    let Ok(value) = tauri_invoke("admin_sign_in", args).await else {
        return false;
    };

    serde_wasm_bindgen::from_value(value).unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn admin_sign_in(username: String, password: String) -> bool {
    local_admin_sign_in(&username, &password)
}

#[cfg(target_arch = "wasm32")]
pub async fn get_investments() -> Vec<Investment> {
    if !tauri_available() {
        return Vec::new();
    }

    let Ok(value) = tauri_invoke("get_investments", wasm_bindgen::JsValue::NULL).await else {
        return Vec::new();
    };

    serde_wasm_bindgen::from_value(value).unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub async fn get_startups() -> Vec<Investment> {
    if !tauri_available() {
        return Vec::new();
    }

    let Ok(value) = tauri_invoke("get_startups", wasm_bindgen::JsValue::NULL).await else {
        return Vec::new();
    };

    serde_wasm_bindgen::from_value(value).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_startups() -> Vec<Investment> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_investments() -> Vec<Investment> {
    Vec::new()
}

#[cfg(target_arch = "wasm32")]
pub async fn submit_startup(submission: StartupSubmission) -> Option<Investment> {
    if !tauri_available() {
        return Some(local_investment(submission));
    }

    let args = serde_wasm_bindgen::to_value(&SubmitStartupArgs { submission });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("submit_startup", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(target_arch = "wasm32")]
pub async fn approve_startup(id: String) -> Option<Investment> {
    review_startup("approve_startup", id).await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn approve_startup(_id: String) -> Option<Investment> {
    None
}

#[cfg(target_arch = "wasm32")]
pub async fn reject_startup(id: String) -> Option<Investment> {
    review_startup("reject_startup", id).await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn reject_startup(_id: String) -> Option<Investment> {
    None
}

#[cfg(target_arch = "wasm32")]
pub async fn register_investor(registration: InvestorRegistration) -> Option<Investor> {
    if !tauri_available() {
        return Some(local_investor(registration));
    }

    let args = serde_wasm_bindgen::to_value(&InvestorRegistrationArgs { registration });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("register_investor", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(target_arch = "wasm32")]
pub async fn investor_sign_in(email: String, password: String) -> Option<Investor> {
    if !tauri_available() {
        if is_developer_login(&email, &password) {
            return Some(developer_investor());
        }

        return Some(Investor {
            id: "investor-1".to_string(),
            name: email.clone(),
            email,
            accredited: false,
        });
    }

    let args = serde_wasm_bindgen::to_value(&AccountSignInArgs { email, password });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("investor_sign_in", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn investor_sign_in(email: String, password: String) -> Option<Investor> {
    if is_developer_login(&email, &password) {
        return Some(developer_investor());
    }

    Some(Investor {
        id: "investor-1".to_string(),
        name: email.clone(),
        email,
        accredited: false,
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn register_business(
    registration: SmallBusinessRegistration,
) -> Option<SmallBusinessAccount> {
    if !tauri_available() {
        return Some(local_business(registration));
    }

    let args = serde_wasm_bindgen::to_value(&SmallBusinessRegistrationArgs { registration });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("register_business", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn register_business(
    registration: SmallBusinessRegistration,
) -> Option<SmallBusinessAccount> {
    Some(local_business(registration))
}

#[cfg(target_arch = "wasm32")]
pub async fn business_sign_in(email: String, password: String) -> Option<SmallBusinessAccount> {
    if !tauri_available() {
        if is_developer_login(&email, &password) {
            return Some(developer_business());
        }

        return Some(SmallBusinessAccount {
            id: "business-1".to_string(),
            company: email.clone(),
            contact: email.clone(),
            email,
        });
    }

    let args = serde_wasm_bindgen::to_value(&AccountSignInArgs { email, password });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("business_sign_in", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn business_sign_in(email: String, password: String) -> Option<SmallBusinessAccount> {
    if is_developer_login(&email, &password) {
        return Some(developer_business());
    }

    Some(SmallBusinessAccount {
        id: "business-1".to_string(),
        company: email.clone(),
        contact: email.clone(),
        email,
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn register_investor(registration: InvestorRegistration) -> Option<Investor> {
    Some(local_investor(registration))
}

#[cfg(target_arch = "wasm32")]
pub async fn sign_nda(
    investor_id: String,
    startup_id: String,
    legal_name: String,
    email: String,
    phone: String,
    address: String,
    initials: String,
) -> Option<NdaSignature> {
    if !tauri_available() {
        return Some(NdaSignature {
            investor_id,
            startup_id,
            legal_name,
            email,
            phone,
            address,
            initials,
            nda_version: "SABER_INVESTOR_NDA_V1".to_string(),
            signed_at: "0".to_string(),
        });
    }

    let args = serde_wasm_bindgen::to_value(&SignNdaArgs {
        investor_id,
        startup_id,
        legal_name,
        email,
        phone,
        address,
        initials,
    });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("sign_nda", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn sign_nda(
    investor_id: String,
    startup_id: String,
    legal_name: String,
    email: String,
    phone: String,
    address: String,
    initials: String,
) -> Option<NdaSignature> {
    Some(NdaSignature {
        investor_id,
        startup_id,
        legal_name,
        email,
        phone,
        address,
        initials,
        nda_version: "SABER_INVESTOR_NDA_V1".to_string(),
        signed_at: "0".to_string(),
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn business_plan_for_investor(investor_id: String, startup_id: String) -> Option<String> {
    if !tauri_available() {
        return None;
    }

    let args = serde_wasm_bindgen::to_value(&BusinessPlanArgs {
        investor_id,
        startup_id,
    });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("business_plan_for_investor", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn business_plan_for_investor(
    _investor_id: String,
    _startup_id: String,
) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
pub async fn request_business_plan(
    investor_id: String,
    startup_id: String,
) -> Option<BusinessPlanRequest> {
    if !tauri_available() {
        return Some(BusinessPlanRequest {
            id: "plan-request-1".to_string(),
            investor_id,
            startup_id,
            status: BusinessPlanRequestStatus::Pending,
            ..BusinessPlanRequest::default()
        });
    }

    let args = serde_wasm_bindgen::to_value(&BusinessPlanArgs {
        investor_id,
        startup_id,
    });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("request_business_plan", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn request_business_plan(
    investor_id: String,
    startup_id: String,
) -> Option<BusinessPlanRequest> {
    Some(BusinessPlanRequest {
        id: "plan-request-1".to_string(),
        investor_id,
        startup_id,
        status: BusinessPlanRequestStatus::Pending,
        ..BusinessPlanRequest::default()
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn list_business_plan_requests_for_business(
    business_id: String,
) -> Vec<BusinessPlanRequest> {
    if !tauri_available() {
        return Vec::new();
    }

    let args = serde_wasm_bindgen::to_value(&BusinessPlanRequestsForBusinessArgs { business_id });
    let Ok(args) = args else {
        return Vec::new();
    };

    let Ok(value) = tauri_invoke("list_business_plan_requests_for_business", args).await else {
        return Vec::new();
    };

    serde_wasm_bindgen::from_value(value).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn list_business_plan_requests_for_business(
    _business_id: String,
) -> Vec<BusinessPlanRequest> {
    Vec::new()
}

#[cfg(target_arch = "wasm32")]
pub async fn approve_business_plan_request(
    business_id: String,
    request_id: String,
) -> Option<BusinessPlanRequest> {
    decide_business_plan_request("approve_business_plan_request", business_id, request_id).await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn approve_business_plan_request(
    business_id: String,
    request_id: String,
) -> Option<BusinessPlanRequest> {
    Some(BusinessPlanRequest {
        id: request_id,
        business_id,
        status: BusinessPlanRequestStatus::Approved,
        ..BusinessPlanRequest::default()
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn decline_business_plan_request(
    business_id: String,
    request_id: String,
) -> Option<BusinessPlanRequest> {
    decide_business_plan_request("decline_business_plan_request", business_id, request_id).await
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn decline_business_plan_request(
    business_id: String,
    request_id: String,
) -> Option<BusinessPlanRequest> {
    Some(BusinessPlanRequest {
        id: request_id,
        business_id,
        status: BusinessPlanRequestStatus::Declined,
        ..BusinessPlanRequest::default()
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn request_connection(
    investor_id: String,
    startup_id: String,
    business_id: String,
    message: String,
) -> Option<ConnectionRequest> {
    if !tauri_available() {
        return Some(ConnectionRequest {
            id: "connection-1".to_string(),
            investor_id,
            startup_id,
            business_id,
            message,
        });
    }

    let args = serde_wasm_bindgen::to_value(&RequestConnectionArgs {
        investor_id,
        startup_id,
        business_id,
        message,
    });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("request_connection", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn request_connection(
    investor_id: String,
    startup_id: String,
    business_id: String,
    message: String,
) -> Option<ConnectionRequest> {
    Some(ConnectionRequest {
        id: "connection-1".to_string(),
        investor_id,
        startup_id,
        business_id,
        message,
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn create_funding_intent(
    investor_id: String,
    startup_id: String,
    amount_cents: u64,
) -> Option<FundingIntent> {
    if !tauri_available() {
        return Some(FundingIntent {
            id: "funding-1".to_string(),
            investor_id,
            startup_id,
            amount_cents,
            status: FundingStatus::PendingReview,
            stripe_checkout_url: None,
        });
    }

    let args = serde_wasm_bindgen::to_value(&CreateFundingIntentArgs {
        investor_id,
        startup_id,
        amount_cents,
    });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke("create_funding_intent", args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn create_funding_intent(
    investor_id: String,
    startup_id: String,
    amount_cents: u64,
) -> Option<FundingIntent> {
    Some(FundingIntent {
        id: "funding-1".to_string(),
        investor_id,
        startup_id,
        amount_cents,
        status: FundingStatus::PendingReview,
        stripe_checkout_url: None,
    })
}

#[cfg(target_arch = "wasm32")]
async fn review_startup(command: &str, id: String) -> Option<Investment> {
    if !tauri_available() {
        return None;
    }

    let args = serde_wasm_bindgen::to_value(&ReviewStartupArgs { id });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke(command, args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(target_arch = "wasm32")]
async fn decide_business_plan_request(
    command: &str,
    business_id: String,
    request_id: String,
) -> Option<BusinessPlanRequest> {
    if !tauri_available() {
        return None;
    }

    let args = serde_wasm_bindgen::to_value(&BusinessPlanRequestDecisionArgs {
        business_id,
        request_id,
    });
    let Ok(args) = args else {
        return None;
    };

    let Ok(value) = tauri_invoke(command, args).await else {
        return None;
    };

    serde_wasm_bindgen::from_value(value).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn submit_startup(submission: StartupSubmission) -> Option<Investment> {
    Some(local_investment(submission))
}

fn local_investment(submission: StartupSubmission) -> Investment {
    Investment {
        id: "startup-1".to_string(),
        business_id: submission.business_id,
        company: submission.company,
        contact: submission.contact,
        email: submission.email,
        stage: submission.stage,
        capital: submission.capital,
        story: submission.story,
        duns_number: submission.duns_number,
        licensing: submission.licensing,
        business_plan: submission.business_plan,
        pictures: submission.pictures,
        status: ReviewStatus::Pending,
    }
}

fn local_investor(registration: InvestorRegistration) -> Investor {
    Investor {
        id: "investor-1".to_string(),
        name: registration.name,
        email: registration.email,
        accredited: registration.accredited,
    }
}

fn local_business(registration: SmallBusinessRegistration) -> SmallBusinessAccount {
    SmallBusinessAccount {
        id: "business-1".to_string(),
        company: registration.company,
        contact: registration.contact,
        email: registration.email,
    }
}

fn is_developer_login(username: &str, password: &str) -> bool {
    username.trim().eq_ignore_ascii_case("developer") && password.trim() == "developer"
}

fn developer_investor() -> Investor {
    Investor {
        id: "developer-investor".to_string(),
        name: "Developer Investor".to_string(),
        email: "developer".to_string(),
        accredited: false,
    }
}

fn developer_business() -> SmallBusinessAccount {
    SmallBusinessAccount {
        id: "developer-business".to_string(),
        company: "Developer Business".to_string(),
        contact: "Developer".to_string(),
        email: "developer".to_string(),
    }
}
