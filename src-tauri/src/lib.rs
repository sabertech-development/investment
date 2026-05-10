mod backend;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(backend::AppStore::default())
    .invoke_handler(tauri::generate_handler![
      backend::admin_sign_in,
      backend::get_investments,
      backend::get_startups,
      backend::submit_startup,
      backend::approve_startup,
      backend::reject_startup,
      backend::register_investor,
      backend::investor_sign_in,
      backend::register_business,
      backend::business_sign_in,
      backend::sign_nda,
      backend::business_plan_for_investor,
      backend::request_business_plan,
      backend::list_business_plan_requests_for_business,
      backend::approve_business_plan_request,
      backend::decline_business_plan_request,
      backend::request_connection,
      backend::create_funding_intent
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
