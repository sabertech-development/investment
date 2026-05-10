use leptos::ev::{Event, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use saberinvestment::backend_client::{
    self, BusinessPlanRequest, BusinessPlanRequestStatus, Investment, Investor,
    InvestorRegistration, ReviewStatus, SmallBusinessAccount, SmallBusinessRegistration,
    StartupSubmission,
};
use saberinvestment::dashboard_nav::DashboardPage;
use saberinvestment::nda::nda_form_complete;

fn main() {
    mount_to_body(App);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Screen {
    SignIn,
    Dashboard,
    InvestorPortal,
    BusinessPortal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PortalRole {
    Admin,
    Investor,
    Business,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuthMode {
    SignIn,
    SignUp,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvestorPage {
    Marketplace,
    DealRoom,
    Funding,
}

const SABER_INVESTOR_NDA_VERSION: &str = "SABER_INVESTOR_NDA_V1";
const SABER_INVESTOR_NDA_TEXT: &str = r#"# SABER INVESTMENT
# INVESTOR NON-DISCLOSURE, CONFIDENTIALITY, AND NON-CIRCUMVENTION AGREEMENT

This Investor Non-Disclosure, Confidentiality, and Non-Circumvention Agreement (this "Agreement") is entered into as of [Effective Date] by and between Saber Investment, a [State/Country] [Entity Type], with its principal address at [Saber Investment Address] ("Saber Investment," "Saber," "Company," or "Disclosing Party"), and [Investor Full Legal Name], with an address at [Investor Address] ("Investor," "Recipient," or "Receiving Party").

Saber Investment and Investor may be referred to individually as a "Party" and collectively as the "Parties."

1. Purpose

Saber Investment operates, develops, or supports a private investment platform, network, marketplace, or related service that allows potential investors to review information regarding privately held startups, early-stage companies, founders, and first-round or early-round funding opportunities.

Investor desires to access certain non-public information, including business plans and related materials, for the limited purpose of evaluating possible investment opportunities in privately held startups introduced, hosted, facilitated, or made available by Saber Investment.

This Agreement governs Investor's access to and use of such confidential information.

2. Privately Held Startup Information

Investor acknowledges that the companies presented through Saber Investment are expected to be privately held startups or early-stage private companies.

Investor further acknowledges that information about privately held startups is often highly sensitive, non-public, and not otherwise available to the general public.

Such information may include early business concepts, founder materials, financial projections, capital needs, proposed investment terms, market research, product plans, technology concepts, trade secrets, customer information, operating plans, and other confidential materials.

3. Confidential Information

For purposes of this Agreement, "Confidential Information" means all non-public, confidential, proprietary, sensitive, or business-related information disclosed or made available to Investor by Saber Investment, any participating startup, any founder, any issuer, or any of their respective affiliates, representatives, advisors, employees, contractors, agents, or service providers.

Confidential Information includes, without limitation: business plans; pitch decks; executive summaries; investor presentations; financial statements; financial projections, forecasts, budgets, and models; revenue, pricing, margin, burn-rate, runway, valuation, and capitalization information; fundraising goals and proposed financing terms; cap tables and ownership information; product plans, product designs, prototypes, technology plans, and software concepts; inventions, intellectual property, trade secrets, know-how, formulas, processes, and methods; market research, competitive analysis, and go-to-market strategy; customer lists, supplier lists, vendor relationships, partner relationships, and sales pipelines; founder, employee, contractor, advisor, and team information; due diligence materials; legal, tax, accounting, technical, operating, or strategic materials; platform data, user data, investor data, startup data, and platform communications; any information marked or identified as confidential, proprietary, sensitive, or private; and any information that a reasonable person would understand to be confidential based on the nature of the information or the circumstances of disclosure.

Confidential Information also includes the existence, status, and content of any discussions, introductions, communications, negotiations, potential investments, potential financing transactions, or potential business relationships involving Saber Investment, Investor, or any participating startup.

4. Third-Party Startup Materials

Investor acknowledges that some Confidential Information may belong to privately held startups, founders, issuers, or other third parties participating in Saber Investment's platform or network.

Investor agrees that such third-party startup materials are protected under this Agreement; Saber Investment may enforce this Agreement with respect to such materials; each participating startup or owner of Confidential Information is an intended third-party beneficiary of this Agreement; and Investor will treat all startup business plans, pitch materials, financial information, and related materials as Confidential Information, whether or not separately marked confidential.

5. Limited Permitted Use

Investor may use Confidential Information solely for the purpose of evaluating a potential investment opportunity made available through Saber Investment.

Investor shall not use Confidential Information for any other purpose, including competing with Saber Investment or any participating startup; copying or commercializing any startup idea, concept, business model, product, service, technology, or strategy; reverse engineering any product, software, process, or technology; developing a competing business based on Confidential Information; soliciting customers, employees, contractors, advisors, suppliers, vendors, or partners of Saber Investment or any participating startup; sharing Confidential Information with unauthorized persons; or circumventing Saber Investment to contact, negotiate with, invest in, finance, acquire, partner with, or transact directly with a participating startup outside Saber Investment's authorized process.

6. Non-Disclosure Obligations

Investor shall keep all Confidential Information strictly confidential; use at least the same degree of care Investor uses to protect Investor's own confidential information, but in no event less than reasonable care; not disclose Confidential Information to any person or entity except as expressly permitted by this Agreement; not copy, download, screenshot, reproduce, distribute, publish, upload, transmit, forward, or otherwise share Confidential Information except as reasonably necessary for the permitted purpose; not post or disclose Confidential Information on social media, public websites, private investor groups, online forums, newsletters, databases, messaging platforms, or other public or semi-public channels; and promptly notify Saber Investment in writing of any unauthorized access, use, disclosure, loss, theft, or compromise of Confidential Information.

7. Permitted Disclosures to Professional Advisors

Investor may disclose Confidential Information only to Investor's legal, tax, accounting, financial, or investment advisors, solely to the extent reasonably necessary for evaluating a potential investment opportunity.

Before disclosing Confidential Information to any advisor, Investor must ensure that the advisor is informed of the confidential nature of the information; is bound by legal, professional, fiduciary, or contractual confidentiality obligations at least as protective as this Agreement; and uses the Confidential Information only for the permitted purpose.

Investor remains responsible for any breach of this Agreement by Investor's advisors or representatives.

8. Non-Circumvention

For a period of [24 months] from the date Investor first receives Confidential Information regarding a participating startup, Investor shall not, without Saber Investment's prior written consent, bypass Saber Investment to contact, negotiate with, invest in, finance, acquire, partner with, or otherwise transact directly with such startup; introduce such startup to any third party for investment, financing, acquisition, partnership, commercial transaction, or similar opportunity outside Saber Investment's authorized process; use Saber Investment's introductions, platform, materials, or relationships to avoid fees, processes, agreements, or approvals required by Saber Investment; or encourage any startup, founder, investor, advisor, or third party to avoid, bypass, or interfere with Saber Investment's relationship with a participating startup or investor.

This section does not prohibit Investor from communicating with a startup through Saber Investment's authorized channels or with Saber Investment's prior written consent.

9. No License or Ownership Transfer

All Confidential Information remains the property of Saber Investment, the applicable participating startup, or the applicable owner of such information. No rights, licenses, assignments, or ownership interests are granted to Investor by disclosure of Confidential Information, whether by implication, estoppel, or otherwise. Investor receives only a limited right to review Confidential Information for the permitted purpose described in this Agreement.

10. No Offer of Securities

Investor acknowledges and agrees that this Agreement does not constitute an offer to sell securities; a solicitation to buy securities; investment advice; legal, tax, accounting, or financial advice; a recommendation by Saber Investment to invest in any startup; or a guarantee of investment returns, liquidity, business performance, or startup success.

Any actual investment shall require separate offering documents, subscription documents, investment agreements, or other transaction documents approved by the applicable startup or issuer.

11. Investor Responsibility and Due Diligence

Investor acknowledges that investments in privately held startups are speculative, risky, illiquid, and may result in the loss of Investor's entire investment. Investor is solely responsible for conducting Investor's own due diligence and consulting Investor's own legal, tax, accounting, and financial advisors before making any investment decision. Investor agrees that Saber Investment does not guarantee the accuracy, completeness, or reliability of any startup's Confidential Information, business plan, projections, financial information, valuation, or investment materials.

12. Investor Eligibility

Investor agrees to provide truthful, accurate, and complete information regarding Investor's identity, investor status, financial sophistication, investment experience, suitability, accreditation status, or other eligibility information if requested by Saber Investment or a participating startup. Investor acknowledges that certain investment opportunities may be available only to investors who satisfy applicable legal, regulatory, financial, sophistication, accreditation, residency, or suitability requirements. Investor shall not access, review, or participate in any opportunity for which Investor is not legally eligible.

13. No Public Disclosure or General Solicitation by Investor

Investor shall not publicly advertise, promote, repost, forward, or otherwise communicate any startup opportunity, business plan, funding opportunity, or Confidential Information to the public or to any unauthorized person. Investor shall not make any public statement regarding Saber Investment's private startup opportunities; any participating startup's fundraising plans; any startup's business plan or financial condition; any proposed securities offering; any investment terms; or any founder or company information obtained through Saber Investment.

14. Exclusions from Confidential Information

Confidential Information does not include information that Investor can prove through written records was publicly available at the time of disclosure through no breach of this Agreement; becomes publicly available after disclosure through no breach of this Agreement; was lawfully known by Investor before disclosure by Saber Investment or a participating startup; is lawfully received by Investor from a third party without breach of any confidentiality obligation; or is independently developed by Investor without use of or reference to Confidential Information.

Information shall not be excluded from protection merely because individual elements of the information are publicly known, unless the specific combination, context, business use, and application of such information are also publicly known.

15. Required Legal Disclosure

If Investor is required by law, court order, subpoena, governmental request, or regulatory process to disclose any Confidential Information, Investor shall, to the extent legally permitted, promptly notify Saber Investment in writing; cooperate with Saber Investment or the applicable startup in seeking confidential treatment, a protective order, or other appropriate remedy; and disclose only the portion of Confidential Information legally required to be disclosed.

16. Return or Destruction of Confidential Information

Upon Saber Investment's request, Investor shall promptly return or destroy all Confidential Information in Investor's possession, custody, or control, including copies, notes, summaries, screenshots, downloads, files, emails, and other records. Investor may retain one archival copy solely to the extent required by law, regulation, professional obligation, or bona fide internal compliance policy, provided that such retained copy remains subject to this Agreement.

17. Security of Platform Access

Investor shall not share login credentials or platform access with any other person; allow unauthorized persons to access Saber Investment's platform or startup materials; attempt to bypass access controls, download restrictions, security features, or confidentiality protections; scrape, archive, copy, export, or mass-download platform content; or use bots, automated tools, or data extraction tools to access Saber Investment's platform or materials.

Investor shall immediately notify Saber Investment of any suspected unauthorized access to Investor's account or any Confidential Information.

18. No Reverse Engineering or Competitive Use

Investor shall not use Confidential Information to reverse engineer, recreate, duplicate, imitate, modify, commercialize, or compete with any product, service, technology, platform, software, business model, process, strategy, or opportunity disclosed by Saber Investment or any participating startup.

19. Non-Solicitation

For a period of [24 months] from the date Investor first receives Confidential Information regarding a participating startup, Investor shall not, without prior written consent from Saber Investment and the applicable startup, directly or indirectly solicit for employment or engagement any founder, employee, contractor, advisor, customer, vendor, supplier, investor, or strategic partner identified through Confidential Information. This section does not prohibit general solicitations not targeted at such persons.

20. Term of Confidentiality

Investor's confidentiality obligations shall begin on the Effective Date and continue for [5 years] after the last disclosure of Confidential Information. For Confidential Information that constitutes a trade secret under applicable law, Investor's obligations shall continue for so long as such information remains a trade secret. The obligations relating to non-circumvention, non-solicitation, ownership, remedies, governing law, and dispute resolution shall survive termination of this Agreement as stated in the applicable sections.

21. Remedies

Investor acknowledges that unauthorized use or disclosure of Confidential Information may cause irreparable harm to Saber Investment and participating startups for which monetary damages may be inadequate. Saber Investment and any applicable participating startup may seek injunctive relief, specific performance, equitable relief, damages, attorneys' fees, costs, and any other remedies available at law or in equity.

22. Indemnification

Investor agrees to indemnify, defend, and hold harmless Saber Investment, participating startups, founders, officers, directors, members, managers, employees, contractors, advisors, agents, affiliates, successors, and assigns from and against any claims, losses, damages, liabilities, costs, and expenses, including reasonable attorneys' fees, arising out of or related to Investor's breach of this Agreement; unauthorized use or disclosure of Confidential Information; Investor's violation of applicable law; Investor's circumvention of Saber Investment; or misrepresentations made by Investor regarding eligibility, identity, accreditation, sophistication, or investment purpose.

23. No Waiver

No failure or delay by Saber Investment in exercising any right under this Agreement shall operate as a waiver of that right. Any waiver must be in writing and signed by Saber Investment.

24. Assignment

Investor may not assign or transfer this Agreement without Saber Investment's prior written consent. Saber Investment may assign this Agreement to an affiliate, successor, acquirer, or assignee in connection with a merger, acquisition, restructuring, sale of assets, financing, or transfer of business operations.

25. Governing Law and Venue

This Agreement shall be governed by and interpreted in accordance with the laws of the State of [Governing Law State], without regard to conflict-of-law principles. Any dispute arising out of or relating to this Agreement shall be brought in the courts located in [County, State], and the Parties consent to personal jurisdiction and venue in such courts.

26. Entire Agreement

This Agreement constitutes the entire agreement between the Parties regarding the subject matter of this Agreement and supersedes all prior or contemporaneous oral or written agreements, understandings, discussions, or communications regarding such subject matter. This Agreement may be amended only in a written document signed by both Parties.

27. Severability

If any provision of this Agreement is held invalid, illegal, or unenforceable, the remaining provisions shall remain in full force and effect. The invalid, illegal, or unenforceable provision shall be modified to the minimum extent necessary to make it valid and enforceable while preserving the Parties' original intent as closely as possible.

28. Electronic Signature

This Agreement may be executed electronically and in counterparts. Electronic signatures, checkbox acceptance, clickwrap acceptance, typed signatures, or signatures through an electronic signature platform shall have the same force and effect as original handwritten signatures, to the fullest extent permitted by applicable law."#;

#[component]
fn App() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (attempted, set_attempted) = signal(false);
    let (checking, set_checking) = signal(false);
    let (failed, set_failed) = signal(false);
    let (screen, set_screen) = signal(Screen::SignIn);
    let (portal_role, set_portal_role) = signal(PortalRole::Admin);
    let (auth_mode, set_auth_mode) = signal(AuthMode::SignIn);
    let (account_name, set_account_name) = signal(String::new());
    let (business_name, set_business_name) = signal(String::new());
    let (business_contact, set_business_contact) = signal(String::new());
    let (investor_session, set_investor_session) = signal(Option::<Investor>::None);
    let (business_session, set_business_session) = signal(Option::<SmallBusinessAccount>::None);
    let (dashboard_page, set_dashboard_page) = signal(DashboardPage::Investments);
    let (investments, set_investments) = signal(Vec::<Investment>::new());
    let (startups, set_startups) = signal(Vec::<Investment>::new());
    let (startup_company, set_startup_company) = signal(String::new());
    let (startup_contact, set_startup_contact) = signal(String::new());
    let (startup_email, set_startup_email) = signal(String::new());
    let (startup_stage, set_startup_stage) = signal(String::new());
    let (startup_capital, set_startup_capital) = signal(String::new());
    let (startup_story, set_startup_story) = signal(String::new());
    let (startup_duns, set_startup_duns) = signal(String::new());
    let (startup_licensing, set_startup_licensing) = signal(String::new());
    let (startup_plan, set_startup_plan) = signal(String::new());
    let (startup_pictures, set_startup_pictures) = signal(String::new());

    let can_submit = move || match (portal_role.get(), auth_mode.get()) {
        (PortalRole::Admin, _) => {
            !email.get().trim().is_empty() && !password.get().trim().is_empty() && !checking.get()
        }
        (PortalRole::Investor, AuthMode::SignIn) => {
            !email.get().trim().is_empty() && !password.get().trim().is_empty() && !checking.get()
        }
        (PortalRole::Investor, AuthMode::SignUp) => {
            !account_name.get().trim().is_empty()
                && !email.get().trim().is_empty()
                && !password.get().trim().is_empty()
                && !checking.get()
        }
        (PortalRole::Business, AuthMode::SignIn) => {
            !email.get().trim().is_empty() && !password.get().trim().is_empty() && !checking.get()
        }
        (PortalRole::Business, AuthMode::SignUp) => {
            !business_name.get().trim().is_empty()
                && !business_contact.get().trim().is_empty()
                && !email.get().trim().is_empty()
                && !password.get().trim().is_empty()
                && !checking.get()
        }
    };
    let readiness = move || {
        if checking.get() {
            "Checking"
        } else if failed.get() {
            "Denied"
        } else if can_submit() {
            "Ready"
        } else if attempted.get() {
            "Check credentials"
        } else {
            "Secure access"
        }
    };

    view! {
        <Show
            when=move || screen.get() != Screen::SignIn
            fallback=move || {
                view! {
                    <main class="shell signin-shell">
                        <section class="market-panel" aria-label="Saber Investment access">
                            <div class="ambient-grid"></div>
                            <div class="market-orbit" aria-hidden="true"></div>
                            <div class="brand-lockup">
                                <span class="brand-mark">"S"</span>
                                <div>
                                    <p class="eyebrow">"Saber Investment"</p>
                                    <h1>"Saber Investment Group"</h1>
                                </div>
                            </div>

                            <div class="signal-stack" aria-hidden="true">
                                <div class="signal-bar signal-bar-primary">
                                    <span>"Private Investment"</span>
                                </div>
                                <div class="signal-bar">
                                    <span>"Promising Startups"</span>
                                </div>
                                <div class="signal-bar">
                                    <span>"Selective Access"</span>
                                </div>
                            </div>
                        </section>

                        <section class="auth-panel" aria-label="Admin credentials">
                            <div class="auth-shine" aria-hidden="true"></div>
                            <div class="status-line">
                                <span class="pulse"></span>
                                <span>{readiness}</span>
                            </div>

                            <div class="role-switch" aria-label="Portal">
                                <button
                                    class:selected=move || portal_role.get() == PortalRole::Admin
                                    type="button"
                                    on:click=move |_| {
                                        set_portal_role.set(PortalRole::Admin);
                                        set_auth_mode.set(AuthMode::SignIn);
                                        set_failed.set(false);
                                    }
                                >
                                    "Admin"
                                </button>
                                <button
                                    class:selected=move || portal_role.get() == PortalRole::Investor
                                    type="button"
                                    on:click=move |_| {
                                        set_portal_role.set(PortalRole::Investor);
                                        set_failed.set(false);
                                    }
                                >
                                    "Investor"
                                </button>
                                <button
                                    class:selected=move || portal_role.get() == PortalRole::Business
                                    type="button"
                                    on:click=move |_| {
                                        set_portal_role.set(PortalRole::Business);
                                        set_failed.set(false);
                                    }
                                >
                                    "Business"
                                </button>
                            </div>

                            <Show when=move || portal_role.get() != PortalRole::Admin>
                                <div class="mode-switch" aria-label="Account mode">
                                    <button
                                        class:selected=move || auth_mode.get() == AuthMode::SignIn
                                        type="button"
                                        on:click=move |_| {
                                            set_auth_mode.set(AuthMode::SignIn);
                                            set_failed.set(false);
                                        }
                                    >
                                        "Sign In"
                                    </button>
                                    <button
                                        class:selected=move || auth_mode.get() == AuthMode::SignUp
                                        type="button"
                                        on:click=move |_| {
                                            set_auth_mode.set(AuthMode::SignUp);
                                            set_failed.set(false);
                                        }
                                    >
                                        "Sign Up"
                                    </button>
                                </div>
                            </Show>

                            <form
                                class="signin-form"
                                on:submit=move |event: SubmitEvent| {
                                    event.prevent_default();
                                    set_attempted.set(true);
                                    set_failed.set(false);

                                    if !can_submit() {
                                        return;
                                    }

                                    let username = email.get();
                                    let password_value = password.get();
                                    set_checking.set(true);

                                    spawn_local(async move {
                                        let signed_in = match (portal_role.get(), auth_mode.get()) {
                                            (PortalRole::Admin, _) => {
                                                if backend_client::admin_sign_in(username, password_value).await {
                                                    set_investments.set(backend_client::get_investments().await);
                                                    set_startups.set(backend_client::get_startups().await);
                                                    set_screen.set(Screen::Dashboard);
                                                    true
                                                } else {
                                                    false
                                                }
                                            }
                                            (PortalRole::Investor, AuthMode::SignIn) => {
                                                if let Some(investor) = backend_client::investor_sign_in(username, password_value).await {
                                                    set_investor_session.set(Some(investor));
                                                    set_investments.set(backend_client::get_investments().await);
                                                    set_screen.set(Screen::InvestorPortal);
                                                    true
                                                } else {
                                                    false
                                                }
                                            }
                                            (PortalRole::Investor, AuthMode::SignUp) => {
                                                let registration = InvestorRegistration {
                                                    name: account_name.get(),
                                                    email: username,
                                                    password: password_value,
                                                    accredited: false,
                                                };
                                                if let Some(investor) = backend_client::register_investor(registration).await {
                                                    set_investor_session.set(Some(investor));
                                                    set_investments.set(backend_client::get_investments().await);
                                                    set_screen.set(Screen::InvestorPortal);
                                                    true
                                                } else {
                                                    false
                                                }
                                            }
                                            (PortalRole::Business, AuthMode::SignIn) => {
                                                if let Some(business) = backend_client::business_sign_in(username, password_value).await {
                                                    set_business_session.set(Some(business));
                                                    set_startups.set(backend_client::get_startups().await);
                                                    set_screen.set(Screen::BusinessPortal);
                                                    true
                                                } else {
                                                    false
                                                }
                                            }
                                            (PortalRole::Business, AuthMode::SignUp) => {
                                                let registration = SmallBusinessRegistration {
                                                    company: business_name.get(),
                                                    contact: business_contact.get(),
                                                    email: username,
                                                    password: password_value,
                                                };
                                                if let Some(business) = backend_client::register_business(registration).await {
                                                    set_business_session.set(Some(business));
                                                    set_startups.set(backend_client::get_startups().await);
                                                    set_screen.set(Screen::BusinessPortal);
                                                    true
                                                } else {
                                                    false
                                                }
                                            }
                                        };
                                        if !signed_in {
                                            set_failed.set(true);
                                        }
                                        set_checking.set(false);
                                    });
                                }
                            >
                                <Show when=move || portal_role.get() == PortalRole::Investor && auth_mode.get() == AuthMode::SignUp>
                                    <label>
                                        <span>"Name"</span>
                                        <input
                                            type="text"
                                            prop:value=account_name
                                            on:input=move |event: Event| set_account_name.set(event_target_value(&event))
                                        />
                                    </label>
                                </Show>
                                <Show when=move || portal_role.get() == PortalRole::Business && auth_mode.get() == AuthMode::SignUp>
                                    <label>
                                        <span>"Company"</span>
                                        <input
                                            type="text"
                                            prop:value=business_name
                                            on:input=move |event: Event| set_business_name.set(event_target_value(&event))
                                        />
                                    </label>
                                    <label>
                                        <span>"Contact"</span>
                                        <input
                                            type="text"
                                            prop:value=business_contact
                                            on:input=move |event: Event| set_business_contact.set(event_target_value(&event))
                                        />
                                    </label>
                                </Show>
                                <label>
                                    <span>{move || if portal_role.get() == PortalRole::Admin { "Username" } else if auth_mode.get() == AuthMode::SignIn { "Username or Email" } else { "Email" }}</span>
                                    <input
                                        autocomplete="username"
                                        type=move || if portal_role.get() == PortalRole::Admin || auth_mode.get() == AuthMode::SignIn { "text" } else { "email" }
                                        prop:value=email
                                        on:input=move |event: Event| {
                                            set_email.set(event_target_value(&event));
                                        }
                                    />
                                </label>

                                <label>
                                    <span>"Password"</span>
                                    <input
                                        autocomplete="current-password"
                                        type="password"
                                        prop:value=password
                                        on:input=move |event: Event| {
                                            set_password.set(event_target_value(&event));
                                        }
                                    />
                                </label>

                                <button class:active=can_submit disabled=move || !can_submit() type="submit">
                                    <span>{move || match (portal_role.get(), auth_mode.get()) {
                                        (PortalRole::Admin, _) | (_, AuthMode::SignIn) => "Enter",
                                        (_, AuthMode::SignUp) => "Create",
                                    }}</span>
                                </button>
                            </form>
                        </section>
                    </main>
                }
            }
        >
            <Show when=move || screen.get() == Screen::Dashboard>
            <Dashboard on_sign_out=move |_| {
                set_email.set(String::new());
                set_password.set(String::new());
                set_attempted.set(false);
                set_failed.set(false);
                set_dashboard_page.set(DashboardPage::Investments);
                set_screen.set(Screen::SignIn);
            }
                dashboard_page=dashboard_page
                set_dashboard_page=set_dashboard_page
                investments=investments
                set_investments=set_investments
                startups=startups
                set_startups=set_startups
                startup_company=startup_company
                set_startup_company=set_startup_company
                startup_contact=startup_contact
                set_startup_contact=set_startup_contact
                startup_email=startup_email
                set_startup_email=set_startup_email
                startup_stage=startup_stage
                set_startup_stage=set_startup_stage
                startup_capital=startup_capital
                set_startup_capital=set_startup_capital
                startup_story=startup_story
                set_startup_story=set_startup_story
                startup_duns=startup_duns
                set_startup_duns=set_startup_duns
                startup_licensing=startup_licensing
                set_startup_licensing=set_startup_licensing
                startup_plan=startup_plan
                set_startup_plan=set_startup_plan
                startup_pictures=startup_pictures
                set_startup_pictures=set_startup_pictures
            />
            </Show>
            <Show when=move || screen.get() == Screen::InvestorPortal>
                <InvestorPortal
                    investor=investor_session
                    investments=investments
                    on_sign_out=move |_| {
                        set_investor_session.set(None);
                        set_email.set(String::new());
                        set_password.set(String::new());
                        set_screen.set(Screen::SignIn);
                    }
                />
            </Show>
            <Show when=move || screen.get() == Screen::BusinessPortal>
                <BusinessPortal
                    business=business_session
                    startups=startups
                    set_startups=set_startups
                    startup_company=startup_company
                    set_startup_company=set_startup_company
                    startup_contact=startup_contact
                    set_startup_contact=set_startup_contact
                    startup_email=startup_email
                    set_startup_email=set_startup_email
                    startup_stage=startup_stage
                    set_startup_stage=set_startup_stage
                    startup_capital=startup_capital
                    set_startup_capital=set_startup_capital
                    startup_story=startup_story
                    set_startup_story=set_startup_story
                    startup_duns=startup_duns
                    set_startup_duns=set_startup_duns
                    startup_licensing=startup_licensing
                    set_startup_licensing=set_startup_licensing
                    startup_plan=startup_plan
                    set_startup_plan=set_startup_plan
                    startup_pictures=startup_pictures
                    set_startup_pictures=set_startup_pictures
                    on_sign_out=move |_| {
                        set_business_session.set(None);
                        set_email.set(String::new());
                        set_password.set(String::new());
                        set_screen.set(Screen::SignIn);
                    }
                />
            </Show>
        </Show>
    }
}

#[component]
fn InvestorPortal<F>(
    investor: ReadSignal<Option<Investor>>,
    investments: ReadSignal<Vec<Investment>>,
    on_sign_out: F,
) -> impl IntoView
where
    F: Fn(()) + Copy + 'static,
{
    let (investor_page, set_investor_page) = signal(InvestorPage::Marketplace);
    let (selected_startup, set_selected_startup) = signal(String::new());
    let (legal_name, set_legal_name) = signal(String::new());
    let (nda_email, set_nda_email) = signal(String::new());
    let (nda_phone, set_nda_phone) = signal(String::new());
    let (nda_address, set_nda_address) = signal(String::new());
    let (nda_initials, set_nda_initials) = signal(String::new());
    let (nda_accepted, set_nda_accepted) = signal(false);
    let (nda_signed, set_nda_signed) = signal(false);
    let (amount, set_amount) = signal(String::new());
    let (business_id, set_business_id) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (notice, set_notice) = signal(String::new());
    let (business_plan, set_business_plan) = signal(String::new());
    let (plan_request_status, set_plan_request_status) =
        signal(Option::<BusinessPlanRequestStatus>::None);
    let signed_in = move || investor.get().is_some();
    let selected_company = move || {
        let selected = selected_startup.get();
        investments
            .get()
            .into_iter()
            .find(|investment| investment.id == selected)
            .map(|investment| investment.company)
            .unwrap_or_else(|| "No startup selected".to_string())
    };
    let funding_dollars = move || amount.get().trim().parse::<u64>().unwrap_or_default();
    let can_fund = move || {
        signed_in()
            && nda_signed.get()
            && !selected_startup.get().is_empty()
            && funding_dollars() > 0
    };
    let can_sign_nda = move || {
        signed_in()
            && !selected_startup.get().is_empty()
            && nda_form_complete(
                &legal_name.get(),
                &nda_email.get(),
                &nda_phone.get(),
                &nda_address.get(),
                &nda_initials.get(),
                nda_accepted.get(),
            )
    };
    let selected_plan = move || {
        let selected = selected_startup.get();
        investments
            .get()
            .into_iter()
            .find(|investment| investment.id == selected)
            .map(|investment| investment.business_plan)
            .unwrap_or_default()
    };

    view! {
        <main class="dashboard-shell portal-shell">
            <aside class="dashboard-rail">
                <div class="brand-lockup compact">
                    <span class="brand-mark">"S"</span>
                    <div>
                        <p class="eyebrow">"Saber Investment"</p>
                        <h1>"Investor"</h1>
                    </div>
                </div>
                <div class="rail-account">
                    <span>"Signed In"</span>
                    <strong>{move || investor.get().map(|item| item.name).unwrap_or_else(|| "Investor".to_string())}</strong>
                </div>
                <nav class="rail-nav investor-rail-nav" aria-label="Investor navigation">
                    <button
                        class:selected=move || investor_page.get() == InvestorPage::Marketplace
                        type="button"
                        on:click=move |_| set_investor_page.set(InvestorPage::Marketplace)
                    >
                        "Marketplace"
                    </button>
                    <button
                        class:selected=move || investor_page.get() == InvestorPage::DealRoom
                        type="button"
                        on:click=move |_| set_investor_page.set(InvestorPage::DealRoom)
                    >
                        "Deal Room"
                    </button>
                    <button
                        class:selected=move || investor_page.get() == InvestorPage::Funding
                        type="button"
                        on:click=move |_| set_investor_page.set(InvestorPage::Funding)
                    >
                        "Funding"
                    </button>
                </nav>
                <button class="ghost-button" type="button" on:click=move |_| on_sign_out(())>
                    "Sign Out"
                </button>
            </aside>
            <section class="dashboard-main">
                <div class="command-bar" aria-label="Investor status">
                    <span>"Private Equity"</span>
                    <span>"NDA Gate"</span>
                    <span>"Small Amounts"</span>
                </div>
                <header class="dashboard-header">
                    <div>
                        <p class="eyebrow">"Private Marketplace"</p>
                        <h2>{move || match investor_page.get() {
                            InvestorPage::Marketplace => "Investment Opportunities",
                            InvestorPage::DealRoom => "Deal Room",
                            InvestorPage::Funding => "Funding",
                        }}</h2>
                    </div>
                </header>

                <section class="metric-grid marketplace-metrics" aria-label="Marketplace summary">
                    <div class="metric-card">
                        <span>"Listings"</span>
                        <strong>{move || investments.get().len().to_string()}</strong>
                    </div>
                    <div class="metric-card">
                        <span>"Capital"</span>
                        <strong>{move || format!("${}", investments.get().iter().map(|item| item.capital).sum::<u64>())}</strong>
                    </div>
                    <div class="metric-card">
                        <span>"Access"</span>
                        <strong>"Private"</strong>
                    </div>
                </section>

                <Show when=move || investor_page.get() == InvestorPage::Marketplace>
                    <section class="tracking-panel marketplace-listings">
                        <div class="tracking-head">
                            <h3>"Listings"</h3>
                        </div>
                        <Show
                            when=move || !investments.get().is_empty()
                            fallback=|| view! { <div class="empty-state"><p>"No approved startups."</p></div> }
                        >
                            <div class="marketplace-grid">
                                <For
                                    each=move || investments.get()
                                    key=|investment| investment.id.clone()
                                    children=move |investment| {
                                        let startup_id = investment.id.clone();
                                        let selected_id = investment.id.clone();
                                        view! {
                                            <article
                                                class="marketplace-card"
                                                class:selected=move || selected_startup.get() == selected_id
                                            >
                                                <div class="marketplace-card-head">
                                                    <span>"Private Listing"</span>
                                                    <strong>{investment.company}</strong>
                                                </div>
                                                <p>{investment.story}</p>
                                                <div class="marketplace-card-meta">
                                                    <div>
                                                        <span class="deal-label">"Stage"</span>
                                                        <span>{investment.stage}</span>
                                                    </div>
                                                    <div>
                                                        <span class="deal-label">"Capital"</span>
                                                        <span>{format!("${}", investment.capital)}</span>
                                                    </div>
                                                </div>
                                                <button
                                                    class="select-button active"
                                                    type="button"
                                                    on:click=move |_| {
                                                        set_selected_startup.set(startup_id.clone());
                                                        set_business_plan.set(String::new());
                                                        set_plan_request_status.set(None);
                                                        set_business_id.set(String::new());
                                                        set_nda_email.set(investor.get().map(|item| item.email).unwrap_or_default());
                                                        set_nda_phone.set(String::new());
                                                        set_nda_address.set(String::new());
                                                        set_nda_initials.set(String::new());
                                                        set_nda_signed.set(false);
                                                        set_nda_accepted.set(false);
                                                        set_notice.set("Deal room opened".to_string());
                                                        set_investor_page.set(InvestorPage::DealRoom);
                                                    }
                                                >
                                                    "Open"
                                                </button>
                                            </article>
                                        }
                                    }
                                />
                            </div>
                        </Show>
                    </section>
                </Show>

                <Show when=move || investor_page.get() == InvestorPage::DealRoom>
                    <section class="tracking-panel deal-room-page">
                        <div class="tracking-head">
                            <h3>"Deal Room"</h3>
                        </div>
                        <section class="investor-console deal-room-body full-room-body">
                            <div class="deal-room-company">
                                <span>"Selected Company"</span>
                                <strong>{selected_company}</strong>
                            </div>
                            <div class="deal-room-steps" id="portfolio">
                                <span>"1 NDA"</span>
                                <span>"2 Plan"</span>
                                <span>"3 Connect"</span>
                            </div>
                            <div class="nda-document">
                                <span>{format!("Investor NDA {}", SABER_INVESTOR_NDA_VERSION)}</span>
                                <div class="nda-scroll">
                                    <p>{SABER_INVESTOR_NDA_TEXT}</p>
                                </div>
                                <div class="nda-signature-grid" aria-label="NDA signature">
                                    <label>
                                        <span>"Investor Name"</span>
                                        <input
                                            type="text"
                                            prop:value=legal_name
                                            on:input=move |event: Event| set_legal_name.set(event_target_value(&event))
                                        />
                                    </label>
                                    <label>
                                        <span>"Email"</span>
                                        <input
                                            inputmode="email"
                                            type="email"
                                            prop:value=nda_email
                                            on:input=move |event: Event| set_nda_email.set(event_target_value(&event))
                                        />
                                    </label>
                                    <label>
                                        <span>"Phone"</span>
                                        <input
                                            inputmode="tel"
                                            type="tel"
                                            prop:value=nda_phone
                                            on:input=move |event: Event| set_nda_phone.set(event_target_value(&event))
                                        />
                                    </label>
                                    <label>
                                        <span>"Address"</span>
                                        <input
                                            type="text"
                                            prop:value=nda_address
                                            on:input=move |event: Event| set_nda_address.set(event_target_value(&event))
                                        />
                                    </label>
                                    <label>
                                        <span>"Initials"</span>
                                        <input
                                            type="text"
                                            prop:value=nda_initials
                                            on:input=move |event: Event| set_nda_initials.set(event_target_value(&event))
                                        />
                                    </label>
                                </div>
                                <label class="check-row">
                                    <input
                                        type="checkbox"
                                        prop:checked=nda_accepted
                                        on:change=move |event| {
                                            set_nda_accepted.set(event_target_checked(&event));
                                        }
                                    />
                                    <span>"I have read and agree"</span>
                                </label>
                            </div>
                            <label>
                                <span>"Business ID"</span>
                                <input
                                    type="text"
                                    prop:value=business_id
                                    on:input=move |event: Event| set_business_id.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Message"</span>
                                <input
                                    type="text"
                                    prop:value=message
                                    on:input=move |event: Event| set_message.set(event_target_value(&event))
                                />
                            </label>
                            <div class="investor-actions">
                                <button
                                    class:active=can_sign_nda
                                    disabled=move || !can_sign_nda()
                                    type="button"
                                    on:click=move |_| {
                                        let Some(investor) = investor.get() else {
                                            return;
                                        };
                                        let startup_id = selected_startup.get();
                                        let name = legal_name.get();
                                        let email = nda_email.get();
                                        let phone = nda_phone.get();
                                        let address = nda_address.get();
                                        let initials = nda_initials.get();
                                        spawn_local(async move {
                                            if backend_client::sign_nda(investor.id, startup_id, name, email, phone, address, initials).await.is_some() {
                                                set_nda_signed.set(true);
                                                set_notice.set("NDA signed. Actions unlocked.".to_string());
                                            } else {
                                                set_notice.set("NDA required".to_string());
                                            }
                                        });
                                    }
                                >
                                    "Sign NDA"
                                </button>
                                <button
                                    class:active=move || signed_in() && nda_signed.get() && !selected_startup.get().is_empty() && plan_request_status.get() != Some(BusinessPlanRequestStatus::Declined)
                                    disabled=move || !signed_in() || !nda_signed.get() || selected_startup.get().is_empty() || plan_request_status.get() == Some(BusinessPlanRequestStatus::Declined)
                                    type="button"
                                    on:click=move |_| {
                                        let Some(investor) = investor.get() else {
                                            return;
                                        };
                                        let startup_id = selected_startup.get();
                                        let plan = selected_plan();
                                        spawn_local(async move {
                                            if plan_request_status.get() == Some(BusinessPlanRequestStatus::Approved) {
                                                if backend_client::business_plan_for_investor(investor.id, startup_id).await.is_some() {
                                                    set_business_plan.set(plan);
                                                    set_notice.set("Plan unlocked".to_string());
                                                } else {
                                                    set_notice.set("Business approval required".to_string());
                                                }
                                            } else if let Some(request) = backend_client::request_business_plan(investor.id, startup_id).await {
                                                let status = request.status;
                                                set_plan_request_status.set(Some(status));
                                                set_notice.set(match status {
                                                    BusinessPlanRequestStatus::Pending => "Plan request pending business approval",
                                                    BusinessPlanRequestStatus::Approved => "Plan request approved",
                                                    BusinessPlanRequestStatus::Declined => "Plan request declined",
                                                }.to_string());
                                            }
                                        });
                                    }
                                >
                                    {move || match plan_request_status.get() {
                                        Some(BusinessPlanRequestStatus::Pending) => "Check Request",
                                        Some(BusinessPlanRequestStatus::Approved) => "Open Business Plan",
                                        Some(BusinessPlanRequestStatus::Declined) => "Declined",
                                        None => "Request Business Plan",
                                    }}
                                </button>
                                <button
                                    class:active=move || signed_in() && nda_signed.get() && !selected_startup.get().is_empty() && !business_id.get().trim().is_empty() && !message.get().trim().is_empty()
                                    disabled=move || !signed_in() || !nda_signed.get() || selected_startup.get().is_empty() || business_id.get().trim().is_empty() || message.get().trim().is_empty()
                                    type="button"
                                    on:click=move |_| {
                                        let Some(investor) = investor.get() else {
                                            return;
                                        };
                                        let startup_id = selected_startup.get();
                                        let business_id = business_id.get();
                                        let note = message.get();
                                        spawn_local(async move {
                                            if backend_client::request_connection(investor.id, startup_id, business_id, note).await.is_some() {
                                                set_notice.set("Connection sent".to_string());
                                            } else {
                                                set_notice.set("Business ID required".to_string());
                                            }
                                        });
                                    }
                                >
                                    "Connect"
                                </button>
                                <button
                                    class:active=move || nda_signed.get() && !selected_startup.get().is_empty()
                                    disabled=move || !nda_signed.get() || selected_startup.get().is_empty()
                                    type="button"
                                    on:click=move |_| set_investor_page.set(InvestorPage::Funding)
                                >
                                    "Funding"
                                </button>
                            </div>
                            <p class="investor-notice">{notice}</p>
                            <Show when=move || !business_plan.get().trim().is_empty()>
                                <div class="plan-lockbox">
                                    <span>"Business Plan"</span>
                                    <a class="plan-link active" href=business_plan target="_blank" rel="noreferrer">
                                        "Open Plan"
                                    </a>
                                </div>
                            </Show>
                        </section>
                    </section>
                </Show>

                <Show when=move || investor_page.get() == InvestorPage::Funding>
                    <section class="tracking-panel funding-page">
                        <div class="tracking-head">
                            <h3>"Funding"</h3>
                        </div>
                        <section class="investor-console deal-room-body full-room-body">
                            <div class="deal-room-company">
                                <span>"Selected Company"</span>
                                <strong>{selected_company}</strong>
                            </div>
                            <div class="deal-room-steps">
                                <span>"1 Amount"</span>
                                <span>"2 Review"</span>
                                <span>"3 Intent"</span>
                            </div>
                            <label>
                                <span>"Amount"</span>
                                <input
                                    inputmode="numeric"
                                    type="number"
                                    min="1"
                                    prop:value=amount
                                    on:input=move |event: Event| set_amount.set(event_target_value(&event))
                                />
                            </label>
                            <div class="investor-actions funding-actions">
                                <button
                                    class:active=move || nda_signed.get() && !selected_startup.get().is_empty()
                                    disabled=move || !nda_signed.get() || selected_startup.get().is_empty()
                                    type="button"
                                    on:click=move |_| set_investor_page.set(InvestorPage::DealRoom)
                                >
                                    "Deal Room"
                                </button>
                                <button
                                    class:active=can_fund
                                    disabled=move || !can_fund()
                                    type="button"
                                    on:click=move |_| {
                                        let Some(investor) = investor.get() else {
                                            return;
                                        };
                                        let startup_id = selected_startup.get();
                                        let amount_cents = funding_dollars() * 100;
                                        spawn_local(async move {
                                            if backend_client::create_funding_intent(investor.id, startup_id, amount_cents).await.is_some() {
                                                set_notice.set("Funding intent pending review".to_string());
                                            } else {
                                                set_notice.set("NDA required".to_string());
                                            }
                                        });
                                    }
                                >
                                    "Create Intent"
                                </button>
                            </div>
                            <p class="investor-notice">{notice}</p>
                        </section>
                    </section>
                </Show>
            </section>
        </main>
    }
}

#[component]
fn BusinessPortal<F>(
    business: ReadSignal<Option<SmallBusinessAccount>>,
    startups: ReadSignal<Vec<Investment>>,
    set_startups: WriteSignal<Vec<Investment>>,
    startup_company: ReadSignal<String>,
    set_startup_company: WriteSignal<String>,
    startup_contact: ReadSignal<String>,
    set_startup_contact: WriteSignal<String>,
    startup_email: ReadSignal<String>,
    set_startup_email: WriteSignal<String>,
    startup_stage: ReadSignal<String>,
    set_startup_stage: WriteSignal<String>,
    startup_capital: ReadSignal<String>,
    set_startup_capital: WriteSignal<String>,
    startup_story: ReadSignal<String>,
    set_startup_story: WriteSignal<String>,
    startup_duns: ReadSignal<String>,
    set_startup_duns: WriteSignal<String>,
    startup_licensing: ReadSignal<String>,
    set_startup_licensing: WriteSignal<String>,
    startup_plan: ReadSignal<String>,
    set_startup_plan: WriteSignal<String>,
    startup_pictures: ReadSignal<String>,
    set_startup_pictures: WriteSignal<String>,
    on_sign_out: F,
) -> impl IntoView
where
    F: Fn(()) + Copy + 'static,
{
    let can_submit_startup = move || {
        !startup_company.get().trim().is_empty()
            && !startup_contact.get().trim().is_empty()
            && !startup_email.get().trim().is_empty()
            && !startup_stage.get().trim().is_empty()
            && !startup_story.get().trim().is_empty()
            && !startup_duns.get().trim().is_empty()
            && !startup_licensing.get().trim().is_empty()
            && !startup_plan.get().trim().is_empty()
    };
    let business_email = move || business.get().map(|item| item.email).unwrap_or_default();
    let business_id_value = move || business.get().map(|item| item.id).unwrap_or_default();
    let (plan_requests, set_plan_requests) = signal(Vec::<BusinessPlanRequest>::new());
    let visible_startups = move || {
        let email = business_email();
        startups
            .get()
            .into_iter()
            .filter(|startup| startup.email == email)
            .collect::<Vec<_>>()
    };

    Effect::new(move |_| {
        if let Some(account) = business.get() {
            if startup_company.get().trim().is_empty() {
                set_startup_company.set(account.company);
            }
            if startup_contact.get().trim().is_empty() {
                set_startup_contact.set(account.contact);
            }
            if startup_email.get().trim().is_empty() {
                set_startup_email.set(account.email);
            }
        }
    });

    Effect::new(move |_| {
        let business_id = business_id_value();
        if business_id.trim().is_empty() {
            return;
        }
        spawn_local(async move {
            set_plan_requests.set(
                backend_client::list_business_plan_requests_for_business(business_id).await,
            );
        });
    });

    view! {
        <main class="dashboard-shell portal-shell">
            <aside class="dashboard-rail">
                <div class="brand-lockup compact">
                    <span class="brand-mark">"S"</span>
                    <div>
                        <p class="eyebrow">"Saber Investment"</p>
                        <h1>"Business"</h1>
                    </div>
                </div>
                <div class="rail-account">
                    <span>"Business ID"</span>
                    <strong>{move || business.get().map(|item| item.id).unwrap_or_else(|| "Not issued".to_string())}</strong>
                </div>
                <button class="ghost-button" type="button" on:click=move |_| on_sign_out(())>
                    "Sign Out"
                </button>
            </aside>
            <section class="dashboard-main">
                <div class="command-bar" aria-label="Business status">
                    <span>"Private Listing"</span>
                    <span>"Admin Review"</span>
                    <span>"Equity Tracking"</span>
                </div>
                <header class="dashboard-header">
                    <div>
                        <p class="eyebrow">"Small Business"</p>
                        <h2>{move || business.get().map(|item| item.company).unwrap_or_else(|| "Business".to_string())}</h2>
                    </div>
                </header>
                <section class="tracking-panel startup-panel">
                    <div class="tracking-head">
                        <h3>"List Business"</h3>
                    </div>
                    <form
                        class="startup-form"
                        on:submit=move |event: SubmitEvent| {
                            event.prevent_default();
                            if !can_submit_startup() {
                                return;
                            }

                            let capital = startup_capital
                                .get()
                                .trim()
                                .parse::<u64>()
                                .unwrap_or_default();
                            let submission = StartupSubmission {
                                business_id: business.get().map(|account| account.id).unwrap_or_default(),
                                company: startup_company.get(),
                                contact: startup_contact.get(),
                                email: startup_email.get(),
                                stage: startup_stage.get(),
                                capital,
                                story: startup_story.get(),
                                duns_number: startup_duns.get(),
                                licensing: startup_licensing.get(),
                                business_plan: startup_plan.get(),
                                pictures: startup_pictures
                                    .get()
                                    .split(',')
                                    .map(|picture| picture.trim().to_string())
                                    .filter(|picture| !picture.is_empty())
                                    .collect(),
                            };

                            spawn_local(async move {
                                if let Some(startup) = backend_client::submit_startup(submission).await {
                                    set_startups.update(|items| items.push(startup));
                                    set_startup_stage.set(String::new());
                                    set_startup_capital.set(String::new());
                                    set_startup_story.set(String::new());
                                    set_startup_duns.set(String::new());
                                    set_startup_licensing.set(String::new());
                                    set_startup_plan.set(String::new());
                                    set_startup_pictures.set(String::new());
                                }
                            });
                        }
                    >
                        <label><span>"Company"</span><input type="text" prop:value=startup_company on:input=move |event: Event| set_startup_company.set(event_target_value(&event)) /></label>
                        <label><span>"Contact"</span><input type="text" prop:value=startup_contact on:input=move |event: Event| set_startup_contact.set(event_target_value(&event)) /></label>
                        <label><span>"Email"</span><input inputmode="email" type="email" prop:value=startup_email on:input=move |event: Event| set_startup_email.set(event_target_value(&event)) /></label>
                        <label><span>"Stage"</span><input type="text" prop:value=startup_stage on:input=move |event: Event| set_startup_stage.set(event_target_value(&event)) /></label>
                        <label><span>"Capital"</span><input inputmode="numeric" type="number" min="0" prop:value=startup_capital on:input=move |event: Event| set_startup_capital.set(event_target_value(&event)) /></label>
                        <label><span>"Story"</span><input type="text" prop:value=startup_story on:input=move |event: Event| set_startup_story.set(event_target_value(&event)) /></label>
                        <label><span>"DUNS"</span><input type="text" prop:value=startup_duns on:input=move |event: Event| set_startup_duns.set(event_target_value(&event)) /></label>
                        <label><span>"Licensing"</span><input type="text" prop:value=startup_licensing on:input=move |event: Event| set_startup_licensing.set(event_target_value(&event)) /></label>
                        <label><span>"Business Plan"</span><input type="text" prop:value=startup_plan on:input=move |event: Event| set_startup_plan.set(event_target_value(&event)) /></label>
                        <label><span>"Pictures"</span><input type="text" prop:value=startup_pictures on:input=move |event: Event| set_startup_pictures.set(event_target_value(&event)) /></label>
                        <button class:active=can_submit_startup disabled=move || !can_submit_startup() type="submit">
                            "Submit"
                        </button>
                    </form>
                </section>

                <section class="tracking-panel">
                    <div class="tracking-head">
                        <h3>"Submissions"</h3>
                    </div>
                    <Show
                        when=move || !visible_startups().is_empty()
                        fallback=|| view! { <div class="empty-state"><p>"No submissions yet."</p></div> }
                    >
                        <div class="investment-list">
                            <For
                                each=visible_startups
                                key=|startup| startup.id.clone()
                                children=move |startup| {
                                    view! {
                                        <article class="investment-row review-row">
                                            <div><span class="deal-label">"Company"</span><strong>{startup.company}</strong></div>
                                            <div><span class="deal-label">"Status"</span><span>{format!("{:?}", startup.status)}</span></div>
                                            <div><span class="deal-label">"DUNS"</span><span>{startup.duns_number}</span></div>
                                            <div><span class="deal-label">"Plan"</span><span>{startup.business_plan}</span></div>
                                        </article>
                                    }
                                }
                            />
                        </div>
                    </Show>
                </section>

                <section class="tracking-panel">
                    <div class="tracking-head">
                        <h3>"Business Plan Requests"</h3>
                    </div>
                    <Show
                        when=move || !plan_requests.get().is_empty()
                        fallback=|| view! { <div class="empty-state"><p>"No plan requests."</p></div> }
                    >
                        <div class="investment-list">
                            <For
                                each=move || plan_requests.get()
                                key=|request| request.id.clone()
                                children=move |request| {
                                    let approve_id = request.id.clone();
                                    let decline_id = request.id.clone();
                                    view! {
                                        <article class="investment-row review-row">
                                            <div><span class="deal-label">"Company"</span><strong>{request.company}</strong></div>
                                            <div><span class="deal-label">"Capital Source"</span><span>{request.investor_name}</span></div>
                                            <div><span class="deal-label">"Status"</span><span>{format!("{:?}", request.status)}</span></div>
                                            <div class="review-actions">
                                                <button
                                                    class:active=move || request.status == BusinessPlanRequestStatus::Pending
                                                    disabled=move || request.status != BusinessPlanRequestStatus::Pending
                                                    type="button"
                                                    on:click=move |_| {
                                                        let business_id = business_id_value();
                                                        let request_id = approve_id.clone();
                                                        spawn_local(async move {
                                                            if let Some(updated) = backend_client::approve_business_plan_request(business_id, request_id).await {
                                                                set_plan_requests.update(|items| {
                                                                    if let Some(item) = items.iter_mut().find(|item| item.id == updated.id) {
                                                                        *item = updated;
                                                                    }
                                                                });
                                                            }
                                                        });
                                                    }
                                                >
                                                    "Approve"
                                                </button>
                                                <button
                                                    class:active=move || request.status == BusinessPlanRequestStatus::Pending
                                                    disabled=move || request.status != BusinessPlanRequestStatus::Pending
                                                    type="button"
                                                    on:click=move |_| {
                                                        let business_id = business_id_value();
                                                        let request_id = decline_id.clone();
                                                        spawn_local(async move {
                                                            if let Some(updated) = backend_client::decline_business_plan_request(business_id, request_id).await {
                                                                set_plan_requests.update(|items| {
                                                                    if let Some(item) = items.iter_mut().find(|item| item.id == updated.id) {
                                                                        *item = updated;
                                                                    }
                                                                });
                                                            }
                                                        });
                                                    }
                                                >
                                                    "Decline"
                                                </button>
                                            </div>
                                        </article>
                                    }
                                }
                            />
                        </div>
                    </Show>
                </section>
            </section>
        </main>
    }
}

#[component]
fn Dashboard<F>(
    on_sign_out: F,
    dashboard_page: ReadSignal<DashboardPage>,
    set_dashboard_page: WriteSignal<DashboardPage>,
    investments: ReadSignal<Vec<Investment>>,
    set_investments: WriteSignal<Vec<Investment>>,
    startups: ReadSignal<Vec<Investment>>,
    set_startups: WriteSignal<Vec<Investment>>,
    startup_company: ReadSignal<String>,
    set_startup_company: WriteSignal<String>,
    startup_contact: ReadSignal<String>,
    set_startup_contact: WriteSignal<String>,
    startup_email: ReadSignal<String>,
    set_startup_email: WriteSignal<String>,
    startup_stage: ReadSignal<String>,
    set_startup_stage: WriteSignal<String>,
    startup_capital: ReadSignal<String>,
    set_startup_capital: WriteSignal<String>,
    startup_story: ReadSignal<String>,
    set_startup_story: WriteSignal<String>,
    startup_duns: ReadSignal<String>,
    set_startup_duns: WriteSignal<String>,
    startup_licensing: ReadSignal<String>,
    set_startup_licensing: WriteSignal<String>,
    startup_plan: ReadSignal<String>,
    set_startup_plan: WriteSignal<String>,
    startup_pictures: ReadSignal<String>,
    set_startup_pictures: WriteSignal<String>,
) -> impl IntoView
where
    F: Fn(()) + Copy + 'static,
{
    let investment_count = move || investments.get().len();
    let capital = move || {
        investments
            .get()
            .iter()
            .map(|investment| investment.capital)
            .sum::<u64>()
    };
    let can_submit_startup = move || {
        !startup_company.get().trim().is_empty()
            && !startup_contact.get().trim().is_empty()
            && !startup_email.get().trim().is_empty()
            && !startup_stage.get().trim().is_empty()
            && !startup_story.get().trim().is_empty()
            && !startup_duns.get().trim().is_empty()
            && !startup_licensing.get().trim().is_empty()
            && !startup_plan.get().trim().is_empty()
    };
    let (investor, set_investor) = signal(Option::<Investor>::None);
    let (investor_name, set_investor_name) = signal(String::new());
    let (investor_email, set_investor_email) = signal(String::new());
    let (investor_accredited, set_investor_accredited) = signal(false);
    let (selected_startup, set_selected_startup) = signal(String::new());
    let (nda_name, set_nda_name) = signal(String::new());
    let (funding_amount, set_funding_amount) = signal(String::new());
    let (connection_message, set_connection_message) = signal(String::new());
    let (investor_notice, set_investor_notice) = signal(String::new());
    let (business_plan, set_business_plan) = signal(String::new());
    let can_register_investor =
        move || !investor_name.get().trim().is_empty() && !investor_email.get().trim().is_empty();
    let has_investor = move || investor.get().is_some();
    let selected_company = move || {
        let selected = selected_startup.get();
        investments
            .get()
            .into_iter()
            .find(|investment| investment.id == selected)
            .map(|investment| investment.company)
            .unwrap_or_else(|| "No startup selected".to_string())
    };
    let funding_dollars = move || {
        funding_amount
            .get()
            .trim()
            .parse::<u64>()
            .unwrap_or_default()
    };
    let can_fund =
        move || has_investor() && !selected_startup.get().is_empty() && funding_dollars() > 0;

    view! {
        <main class="dashboard-shell">
            <aside class="dashboard-rail">
                <div class="brand-lockup compact">
                    <span class="brand-mark">"S"</span>
                    <div>
                        <p class="eyebrow">"Saber Investment"</p>
                        <h1>"Admin"</h1>
                    </div>
                </div>

                <nav class="rail-nav" aria-label="Admin navigation">
                    <button
                        class:selected=move || dashboard_page.get() == DashboardPage::Investments
                        type="button"
                        on:click=move |_| set_dashboard_page.set(DashboardPage::Investments)
                    >
                        "Investments"
                    </button>
                    <button
                        class:selected=move || dashboard_page.get() == DashboardPage::Investors
                        type="button"
                        on:click=move |_| set_dashboard_page.set(DashboardPage::Investors)
                    >
                        "Investors"
                    </button>
                    <button
                        class:selected=move || dashboard_page.get() == DashboardPage::Startups
                        type="button"
                        on:click=move |_| set_dashboard_page.set(DashboardPage::Startups)
                    >
                        "Startups"
                    </button>
                    <button
                        class:selected=move || dashboard_page.get() == DashboardPage::Review
                        type="button"
                        on:click=move |_| set_dashboard_page.set(DashboardPage::Review)
                    >
                        "Review"
                    </button>
                </nav>

                <button class="ghost-button" type="button" on:click=move |_| on_sign_out(())>
                    "Sign Out"
                </button>
            </aside>

            <section class="dashboard-main">
                <div class="command-bar" aria-label="Dashboard status">
                    <span>"Private Markets"</span>
                    <span>"Secure Session"</span>
                    <span>"Admin Console"</span>
                </div>

                <header class="dashboard-header">
                    <div>
                        <p class="eyebrow">"Investment Tracking"</p>
                        <h2>{move || dashboard_page.get().title()}</h2>
                    </div>
                    <Show when=move || dashboard_page.get() == DashboardPage::Investments>
                        <button
                            class="primary-action"
                            type="button"
                            on:click=move |_| set_dashboard_page.set(DashboardPage::Startups)
                        >
                            "List Startup"
                        </button>
                    </Show>
                    <Show when=move || dashboard_page.get() == DashboardPage::Investors>
                        <button
                            class="primary-action"
                            type="button"
                            on:click=move |_| set_dashboard_page.set(DashboardPage::Startups)
                        >
                            "List Startup"
                        </button>
                    </Show>
                </header>

                <Show
                    when=move || dashboard_page.get() != DashboardPage::Investments
                    fallback=move || {
                        view! {
                            <section class="metric-grid premium-grid" aria-label="Investment summary">
                                <div class="metric-card">
                                    <span>"Capital"</span>
                                    <strong>{move || format!("${}", capital())}</strong>
                                </div>
                                <div class="metric-card">
                                    <span>"Startups"</span>
                                    <strong>{move || investment_count().to_string()}</strong>
                                </div>
                                <div class="metric-card">
                                    <span>"Rounds"</span>
                                    <strong>{move || investment_count().to_string()}</strong>
                                </div>
                            </section>

                            <section class="tracking-panel" id="investments">
                                <div class="tracking-head">
                                    <h3>"Investments"</h3>
                                </div>
                                <Show
                                    when=move || !investments.get().is_empty()
                                    fallback=|| {
                                        view! {
                                            <div class="empty-state">
                                                <p>"No investments yet."</p>
                                            </div>
                                        }
                                    }
                                >
                                    <div class="investment-list">
                                        <For
                                            each=move || investments.get()
                                            key=|investment| investment.id.clone()
                                            children=move |investment| {
                                                view! {
                                                    <article class="investment-row">
                                                        <div>
                                                            <span class="deal-label">"Startup"</span>
                                                            <strong>{investment.company}</strong>
                                                        </div>
                                                        <div>
                                                            <span class="deal-label">"Stage"</span>
                                                            <span>{investment.stage}</span>
                                                        </div>
                                                        <div>
                                                            <span class="deal-label">"Capital"</span>
                                                            <span>{format!("${}", investment.capital)}</span>
                                                        </div>
                                                        <div>
                                                            <span class="deal-label">"Contact"</span>
                                                            <span>{investment.contact}</span>
                                                        </div>
                                                    </article>
                                                }
                                            }
                                        />
                                    </div>
                                </Show>
                            </section>
                        }
                    }
                >
                    <Show
                        when=move || dashboard_page.get() == DashboardPage::Review
                        fallback=move || {
                            view! {
                                <Show
                                    when=move || dashboard_page.get() == DashboardPage::Startups
                                    fallback=move || {
                                        view! {
                                            <section class="tracking-panel investor-panel" id="investors">
                                                <div class="tracking-head">
                                                    <h3>"Investor Access"</h3>
                                                </div>
                                                <div class="investor-layout">
                                                    <form
                                                        class="startup-form investor-form"
                                                        on:submit=move |event: SubmitEvent| {
                                                            event.prevent_default();
                                                            if !can_register_investor() {
                                                                return;
                                                            }

                                                            let registration = InvestorRegistration {
                                                                name: investor_name.get(),
                                                                email: investor_email.get(),
                                                                password: "investor".to_string(),
                                                                accredited: investor_accredited.get(),
                                                            };

                                                            spawn_local(async move {
                                                                if let Some(registered) = backend_client::register_investor(registration).await {
                                                                    set_investor.set(Some(registered));
                                                                    set_investor_notice.set("Investor ready".to_string());
                                                                }
                                                            });
                                                        }
                                                    >
                                                        <label>
                                                            <span>"Name"</span>
                                                            <input
                                                                type="text"
                                                                prop:value=investor_name
                                                                on:input=move |event: Event| set_investor_name.set(event_target_value(&event))
                                                            />
                                                        </label>
                                                        <label>
                                                            <span>"Email"</span>
                                                            <input
                                                                inputmode="email"
                                                                type="email"
                                                                prop:value=investor_email
                                                                on:input=move |event: Event| set_investor_email.set(event_target_value(&event))
                                                            />
                                                        </label>
                                                        <label class="check-row">
                                                            <input
                                                                type="checkbox"
                                                                prop:checked=investor_accredited
                                                                on:change=move |event| {
                                                                    set_investor_accredited.set(event_target_checked(&event));
                                                                }
                                                            />
                                                            <span>"Accredited"</span>
                                                        </label>
                                                        <button class:active=can_register_investor disabled=move || !can_register_investor() type="submit">
                                                            "Register"
                                                        </button>
                                                    </form>

                                                    <section class="investor-console">
                                                        <div class="investor-status">
                                                            <span>"Investor"</span>
                                                            <strong>{move || investor.get().map(|item| item.name).unwrap_or_else(|| "Not registered".to_string())}</strong>
                                                        </div>
                                                        <div class="investor-status">
                                                            <span>"Startup"</span>
                                                            <strong>{selected_company}</strong>
                                                        </div>
                                                        <label>
                                                            <span>"Legal Name"</span>
                                                            <input
                                                                type="text"
                                                                prop:value=nda_name
                                                                on:input=move |event: Event| set_nda_name.set(event_target_value(&event))
                                                            />
                                                        </label>
                                                        <label>
                                                            <span>"Amount"</span>
                                                            <input
                                                                inputmode="decimal"
                                                                type="number"
                                                                min="1"
                                                                step="1"
                                                                prop:value=funding_amount
                                                                on:input=move |event: Event| set_funding_amount.set(event_target_value(&event))
                                                            />
                                                        </label>
                                                        <label>
                                                            <span>"Message"</span>
                                                            <input
                                                                type="text"
                                                                prop:value=connection_message
                                                                on:input=move |event: Event| set_connection_message.set(event_target_value(&event))
                                                            />
                                                        </label>
                                                        <div class="investor-actions">
                                                            <button
                                                                class:active=move || has_investor() && !selected_startup.get().is_empty() && !nda_name.get().trim().is_empty()
                                                                disabled=move || !has_investor() || selected_startup.get().is_empty() || nda_name.get().trim().is_empty()
                                                                type="button"
                                                                on:click=move |_| {
                                                                    let Some(investor) = investor.get() else {
                                                                        return;
                                                                    };
                                                                    let startup_id = selected_startup.get();
                                                                    let legal_name = nda_name.get();
                                                                    let email = investor.email.clone();
                                                                    spawn_local(async move {
                                                                        if backend_client::sign_nda(
                                                                            investor.id,
                                                                            startup_id,
                                                                            legal_name,
                                                                            email,
                                                                            "Admin entry".to_string(),
                                                                            "Admin entry".to_string(),
                                                                            "AE".to_string(),
                                                                        ).await.is_some() {
                                                                            set_investor_notice.set("NDA signed".to_string());
                                                                        } else {
                                                                            set_investor_notice.set("NDA required".to_string());
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "Sign NDA"
                                                            </button>
                                                            <button
                                                                class:active=move || has_investor() && !selected_startup.get().is_empty()
                                                                disabled=move || !has_investor() || selected_startup.get().is_empty()
                                                                type="button"
                                                                on:click=move |_| {
                                                                    let Some(investor) = investor.get() else {
                                                                        return;
                                                                    };
                                                                    let startup_id = selected_startup.get();
                                                                    spawn_local(async move {
                                                                        if let Some(plan) = backend_client::business_plan_for_investor(investor.id, startup_id).await {
                                                                            set_business_plan.set(plan);
                                                                            set_investor_notice.set("Plan unlocked".to_string());
                                                                        } else {
                                                                            set_investor_notice.set("NDA required".to_string());
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "View Plan"
                                                            </button>
                                                            <button
                                                                class:active=move || has_investor() && !selected_startup.get().is_empty() && !connection_message.get().trim().is_empty()
                                                                disabled=move || !has_investor() || selected_startup.get().is_empty() || connection_message.get().trim().is_empty()
                                                                type="button"
                                                                on:click=move |_| {
                                                                    let Some(investor) = investor.get() else {
                                                                        return;
                                                                    };
                                                                    let startup_id = selected_startup.get();
                                                                    let business_id = investments
                                                                        .get()
                                                                        .into_iter()
                                                                        .find(|investment| investment.id == startup_id)
                                                                        .map(|investment| investment.business_id)
                                                                        .unwrap_or_default();
                                                                    let message = connection_message.get();
                                                                    spawn_local(async move {
                                                                        if backend_client::request_connection(investor.id, startup_id, business_id, message).await.is_some() {
                                                                            set_investor_notice.set("Connection sent".to_string());
                                                                        } else {
                                                                            set_investor_notice.set("NDA required".to_string());
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "Connect"
                                                            </button>
                                                            <button
                                                                class:active=can_fund
                                                                disabled=move || !can_fund()
                                                                type="button"
                                                                on:click=move |_| {
                                                                    let Some(investor) = investor.get() else {
                                                                        return;
                                                                    };
                                                                    let startup_id = selected_startup.get();
                                                                    let amount_cents = funding_dollars() * 100;
                                                                    spawn_local(async move {
                                                                        if backend_client::create_funding_intent(investor.id, startup_id, amount_cents).await.is_some() {
                                                                            set_investor_notice.set("Funding intent pending review".to_string());
                                                                        } else {
                                                                            set_investor_notice.set("NDA required".to_string());
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "Fund"
                                                            </button>
                                                        </div>
                                                        <p class="investor-notice">{investor_notice}</p>
                                                        <Show when=move || !business_plan.get().trim().is_empty()>
                                                            <div class="plan-lockbox">
                                                                <span>"Business Plan"</span>
                                                                <strong>{business_plan}</strong>
                                                            </div>
                                                        </Show>
                                                    </section>
                                                </div>

                                                <Show
                                                    when=move || !investments.get().is_empty()
                                                    fallback=|| {
                                                        view! {
                                                            <div class="empty-state">
                                                                <p>"No approved startups."</p>
                                                            </div>
                                                        }
                                                    }
                                                >
                                                    <div class="investment-list">
                                                        <For
                                                            each=move || investments.get()
                                                            key=|investment| investment.id.clone()
                                                            children=move |investment| {
                                                                let startup_id = investment.id.clone();
                                                                view! {
                                                                    <article class="investment-row investor-row">
                                                                        <div>
                                                                            <span class="deal-label">"Startup"</span>
                                                                            <strong>{investment.company}</strong>
                                                                        </div>
                                                                        <div>
                                                                            <span class="deal-label">"Stage"</span>
                                                                            <span>{investment.stage}</span>
                                                                        </div>
                                                                        <div>
                                                                            <span class="deal-label">"Capital"</span>
                                                                            <span>{format!("${}", investment.capital)}</span>
                                                                        </div>
                                                                        <div>
                                                                            <button
                                                                                class="select-button active"
                                                                                type="button"
                                                                                on:click=move |_| {
                                                                                    set_selected_startup.set(startup_id.clone());
                                                                                    set_business_plan.set(String::new());
                                                                                    set_investor_notice.set("Startup selected".to_string());
                                                                                }
                                                                            >
                                                                                "Select"
                                                                            </button>
                                                                        </div>
                                                                    </article>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                </Show>
                                            </section>
                                        }
                                    }
                                >
                    <section class="tracking-panel startup-panel" id="startups">
                        <div class="tracking-head">
                            <h3>"List Startup"</h3>
                        </div>
                        <div class="startup-layout">
                        <form
                            class="startup-form"
                            on:submit=move |event: SubmitEvent| {
                                event.prevent_default();
                                if !can_submit_startup() {
                                    return;
                                }

                                let capital = startup_capital
                                    .get()
                                    .trim()
                                    .parse::<u64>()
                                    .unwrap_or_default();
                                let submission = StartupSubmission {
                                    business_id: String::new(),
                                    company: startup_company.get(),
                                    contact: startup_contact.get(),
                                    email: startup_email.get(),
                                    stage: startup_stage.get(),
                                    capital,
                                    story: startup_story.get(),
                                    duns_number: startup_duns.get(),
                                    licensing: startup_licensing.get(),
                                    business_plan: startup_plan.get(),
                                    pictures: startup_pictures
                                        .get()
                                        .split(',')
                                        .map(|picture| picture.trim().to_string())
                                        .filter(|picture| !picture.is_empty())
                                        .collect(),
                                };

                                spawn_local(async move {
                                    if let Some(startup) = backend_client::submit_startup(submission).await {
                                        set_startups.update(|items| items.push(startup));
                                        set_startup_company.set(String::new());
                                        set_startup_contact.set(String::new());
                                        set_startup_email.set(String::new());
                                        set_startup_stage.set(String::new());
                                        set_startup_capital.set(String::new());
                                        set_startup_story.set(String::new());
                                        set_startup_duns.set(String::new());
                                        set_startup_licensing.set(String::new());
                                        set_startup_plan.set(String::new());
                                        set_startup_pictures.set(String::new());
                                        set_dashboard_page.set(DashboardPage::Review);
                                    }
                                });
                            }
                        >
                            <label>
                                <span>"Startup"</span>
                                <input
                                    type="text"
                                    prop:value=startup_company
                                    on:input=move |event: Event| set_startup_company.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Contact"</span>
                                <input
                                    type="text"
                                    prop:value=startup_contact
                                    on:input=move |event: Event| set_startup_contact.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Email"</span>
                                <input
                                    inputmode="email"
                                    type="email"
                                    prop:value=startup_email
                                    on:input=move |event: Event| set_startup_email.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Stage"</span>
                                <input
                                    type="text"
                                    prop:value=startup_stage
                                    on:input=move |event: Event| set_startup_stage.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Capital"</span>
                                <input
                                    inputmode="numeric"
                                    type="number"
                                    min="0"
                                    prop:value=startup_capital
                                    on:input=move |event: Event| set_startup_capital.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Story"</span>
                                <input
                                    type="text"
                                    prop:value=startup_story
                                    on:input=move |event: Event| set_startup_story.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"DUNS"</span>
                                <input
                                    type="text"
                                    prop:value=startup_duns
                                    on:input=move |event: Event| set_startup_duns.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Licensing"</span>
                                <input
                                    type="text"
                                    prop:value=startup_licensing
                                    on:input=move |event: Event| set_startup_licensing.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Business Plan"</span>
                                <input
                                    type="text"
                                    prop:value=startup_plan
                                    on:input=move |event: Event| set_startup_plan.set(event_target_value(&event))
                                />
                            </label>
                            <label>
                                <span>"Pictures"</span>
                                <input
                                    type="text"
                                    prop:value=startup_pictures
                                    on:input=move |event: Event| set_startup_pictures.set(event_target_value(&event))
                                />
                            </label>
                            <button class:active=can_submit_startup disabled=move || !can_submit_startup() type="submit">
                                "Submit"
                            </button>
                        </form>
                        <aside class="startup-preview" aria-label="Startup submission preview">
                            <span>"Submission"</span>
                            <strong>{move || {
                                let company = startup_company.get();
                                if company.trim().is_empty() {
                                    "Not Started".to_string()
                                } else {
                                    company
                                }
                            }}</strong>
                            <p>{move || {
                                let stage = startup_stage.get();
                                if stage.trim().is_empty() {
                                    "Awaiting stage".to_string()
                                } else {
                                    stage
                                }
                            }}</p>
                        </aside>
                        </div>
                    </section>
                                </Show>
                            }
                        }
                    >
                        <section class="tracking-panel" id="review">
                            <div class="tracking-head">
                                <h3>"Admin Review"</h3>
                            </div>
                            <Show
                                when=move || !startups.get().is_empty()
                                fallback=|| {
                                    view! {
                                        <div class="empty-state">
                                            <p>"No startups pending."</p>
                                        </div>
                                    }
                                }
                            >
                                <div class="investment-list">
                                    <For
                                        each=move || startups.get()
                                        key=|startup| startup.id.clone()
                                        children=move |startup| {
                                            let approve_id = startup.id.clone();
                                            let reject_id = startup.id.clone();
                                            view! {
                                                <article class="investment-row review-row">
                                                    <div>
                                                        <span class="deal-label">"Company"</span>
                                                        <strong>{startup.company}</strong>
                                                    </div>
                                                    <div>
                                                        <span class="deal-label">"Status"</span>
                                                        <span>{format!("{:?}", startup.status)}</span>
                                                    </div>
                                                    <div>
                                                        <span class="deal-label">"DUNS"</span>
                                                        <span>{startup.duns_number}</span>
                                                    </div>
                                                    <div>
                                                        <span class="deal-label">"Plan"</span>
                                                        <span>{startup.business_plan}</span>
                                                    </div>
                                                    <p class="review-story">{startup.story}</p>
                                                    <div class="review-actions">
                                                        <button
                                                            class:active=move || startup.status == ReviewStatus::Pending
                                                            disabled=move || startup.status != ReviewStatus::Pending
                                                            type="button"
                                                            on:click=move |_| {
                                                                let id = approve_id.clone();
                                                                spawn_local(async move {
                                                                    if let Some(updated) = backend_client::approve_startup(id).await {
                                                                        set_startups.update(|items| {
                                                                            if let Some(item) = items.iter_mut().find(|item| item.id == updated.id) {
                                                                                *item = updated.clone();
                                                                            }
                                                                        });
                                                                        set_investments.set(backend_client::get_investments().await);
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            "Approve"
                                                        </button>
                                                        <button
                                                            class:active=move || startup.status == ReviewStatus::Pending
                                                            disabled=move || startup.status != ReviewStatus::Pending
                                                            type="button"
                                                            on:click=move |_| {
                                                                let id = reject_id.clone();
                                                                spawn_local(async move {
                                                                    if let Some(updated) = backend_client::reject_startup(id).await {
                                                                        set_startups.update(|items| {
                                                                            if let Some(item) = items.iter_mut().find(|item| item.id == updated.id) {
                                                                                *item = updated;
                                                                            }
                                                                        });
                                                                        set_investments.set(backend_client::get_investments().await);
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            "Reject"
                                                        </button>
                                                    </div>
                                                </article>
                                            }
                                        }
                                    />
                                </div>
                            </Show>
                        </section>
                    </Show>
                </Show>
            </section>
        </main>
    }
}
