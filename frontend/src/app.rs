use leptos::*;
use gloo_net::http::Request;
use shared::{VisionPayload, GoalsPayload, Reminder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Home,
    Mission,
    Vision,
    Goals,
    Reminder,
}

#[component]
pub fn App() -> impl IntoView {
    // Current date formatted as "29 June 26"
    let date = js_sys::Date::new_0();
    let day = date.get_date();
    let month_idx = date.get_month() as usize;
    let year = date.get_full_year() % 100;
    let months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    let month_name = months.get(month_idx).copied().unwrap_or("January");
    let formatted_str = format!("{} {} {:02}", day, month_name, year);
    let (formatted_date, _) = create_signal(formatted_str);

    // Current Active Tab
    let (active_tab, set_active_tab) = create_signal(Tab::Home);

    // Backend state signals
    let (vision_data, set_vision_data) = create_signal(VisionPayload {
        vision: String::new(),
        mission: String::new(),
    });
    let (goals_data, set_goals_data) = create_signal(GoalsPayload {
        yearly: String::new(),
        quarterly: String::new(),
        monthly: String::new(),
        weekly: String::new(),
    });
    let (reminders, set_reminders) = create_signal(Vec::<Reminder>::new());

    // Edit form signals
    let (edit_vision, set_edit_vision) = create_signal(String::new());
    let (edit_mission, set_edit_mission) = create_signal(String::new());
    let (edit_yearly, set_edit_yearly) = create_signal(String::new());
    let (edit_quarterly, set_edit_quarterly) = create_signal(String::new());
    let (edit_monthly, set_edit_monthly) = create_signal(String::new());
    let (edit_weekly, set_edit_weekly) = create_signal(String::new());
    let (new_reminder_text, set_new_reminder_text) = create_signal(String::new());

    // Load function to fetch all dashboard data
    let load_all_data = move || {
        // Fetch Vision/Mission
        spawn_local(async move {
            if let Ok(resp) = Request::get("http://127.0.0.1:3000/api/vision").send().await {
                if let Ok(data) = resp.json::<VisionPayload>().await {
                    set_vision_data.set(data.clone());
                    set_edit_vision.set(data.vision);
                    set_edit_mission.set(data.mission);
                }
            }
        });

        // Fetch Goals
        spawn_local(async move {
            if let Ok(resp) = Request::get("http://127.0.0.1:3000/api/goals").send().await {
                if let Ok(data) = resp.json::<GoalsPayload>().await {
                    set_goals_data.set(data.clone());
                    set_edit_yearly.set(data.yearly);
                    set_edit_quarterly.set(data.quarterly);
                    set_edit_monthly.set(data.monthly);
                    set_edit_weekly.set(data.weekly);
                }
            }
        });

        // Fetch Reminders
        spawn_local(async move {
            if let Ok(resp) = Request::get("http://127.0.0.1:3000/api/reminders").send().await {
                if let Ok(data) = resp.json::<Vec<Reminder>>().await {
                    set_reminders.set(data);
                }
            }
        });
    };

    // Load data on startup
    create_effect(move |_| {
        load_all_data();
    });

    // Save Vision Actions
    let handle_save_vision = move |_| {
        let payload = VisionPayload {
            vision: edit_vision.get(),
            mission: edit_mission.get(),
        };
        spawn_local(async move {
            let _ = Request::put("http://127.0.0.1:3000/api/vision")
                .json(&payload)
                .unwrap()
                .send()
                .await;
            load_all_data();
            set_active_tab.set(Tab::Home);
        });
    };

    // Save Goals Actions
    let handle_save_goals = move |_| {
        let payload = GoalsPayload {
            yearly: edit_yearly.get(),
            quarterly: edit_quarterly.get(),
            monthly: edit_monthly.get(),
            weekly: edit_weekly.get(),
        };
        spawn_local(async move {
            let _ = Request::put("http://127.0.0.1:3000/api/goals")
                .json(&payload)
                .unwrap()
                .send()
                .await;
            load_all_data();
            set_active_tab.set(Tab::Home);
        });
    };

    // Add Reminder Action
    let handle_add_reminder = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        let text = new_reminder_text.get().trim().to_string();
        if text.is_empty() { return; }

        let payload = Reminder {
            id: None,
            text,
            is_completed: false,
            created_at: None,
        };

        spawn_local(async move {
            let _ = Request::post("http://127.0.0.1:3000/api/reminders")
                .json(&payload)
                .unwrap()
                .send()
                .await;
            set_new_reminder_text.set(String::new());
            load_all_data();
        });
    };

    // Toggle Reminder Action
    let handle_toggle_reminder = move |reminder: Reminder| {
        let mut updated = reminder.clone();
        updated.is_completed = !reminder.is_completed;
        let id = reminder.id.unwrap_or(0);

        spawn_local(async move {
            let _ = Request::put(&format!("http://127.0.0.1:3000/api/reminders/{}", id))
                .json(&updated)
                .unwrap()
                .send()
                .await;
            load_all_data();
        });
    };

    // Delete Reminder Action
    let handle_delete_reminder = move |id: i64| {
        spawn_local(async move {
            let _ = Request::delete(&format!("http://127.0.0.1:3000/api/reminders/{}", id))
                .send()
                .await;
            load_all_data();
        });
    };

    view! {
        <div class="app-container">
            // Top Header
            <header class="app-header">
                <h1>"REMEMBER"</h1>
                <div class="card-subtitle">"Personal Workspace"</div>
            </header>

            // Main Content Area (dynamic based on Active Tab)
            <main class="app-content">
                {move || match active_tab.get() {
                    Tab::Home => view! {
                        <div class="dashboard-welcome" style="display: flex; justify-content: space-between; align-items: flex-end; width: 100%;">
                            <div>
                                <h2>"Hello, Sadiq"</h2>
                                <p>"Here is your focus dashboard for today."</p>
                            </div>
                            <div style="font-size: 14px; font-weight: 700; color: var(--accent-primary); font-family: 'Outfit', sans-serif; letter-spacing: 0.5px; padding-bottom: 2px;">
                                {formatted_date}
                            </div>
                        </div>

                        // Goals Summary
                        <div class="glass-card" on:click=move |_| set_active_tab.set(Tab::Goals)>
                            <div class="card-header">
                                <span class="card-title">"Active Goals"</span>
                                <span class="card-subtitle">"Tap to edit"</span>
                            </div>
                            <div style="display: flex; flex-direction: column; gap: 16px; margin-top: 10px;">
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <span class="goal-timeframe-tag tag-yearly">"Yearly"</span>
                                    </div>
                                    <div style="font-size: 13px; color: #e2e8f0; padding-left: 4px;">
                                        {move || if goals_data.get().yearly.is_empty() { "Not set".to_string() } else { goals_data.get().yearly }}
                                    </div>
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <span class="goal-timeframe-tag tag-quarterly">"Quarter"</span>
                                    </div>
                                    <div style="font-size: 13px; color: #e2e8f0; padding-left: 4px;">
                                        {move || if goals_data.get().quarterly.is_empty() { "Not set".to_string() } else { goals_data.get().quarterly }}
                                    </div>
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <span class="goal-timeframe-tag tag-monthly">"Monthly"</span>
                                    </div>
                                    <div style="font-size: 13px; color: #e2e8f0; padding-left: 4px;">
                                        {move || if goals_data.get().monthly.is_empty() { "Not set".to_string() } else { goals_data.get().monthly }}
                                    </div>
                                </div>
                                <div style="display: flex; flex-direction: column; gap: 4px;">
                                    <div style="display: flex; align-items: center; gap: 8px;">
                                        <span class="goal-timeframe-tag tag-weekly">"Weekly"</span>
                                    </div>
                                    <div style="font-size: 13px; color: #e2e8f0; padding-left: 4px;">
                                        {move || if goals_data.get().weekly.is_empty() { "Not set".to_string() } else { goals_data.get().weekly }}
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Mission & Vision Preview
                        <div class="glass-card large-display-card vision-glow" on:click=move |_| set_active_tab.set(Tab::Vision)>
                            <div class="card-header">
                                <span class="card-title">"Vision Statement"</span>
                                <span class="card-subtitle">"Tap to edit"</span>
                            </div>
                            <div class="card-content">
                                {move || if vision_data.get().vision.is_empty() {
                                    "No vision statement set. Tap to add your guiding star.".to_string()
                                } else {
                                    vision_data.get().vision
                                }}
                            </div>
                        </div>

                        <div class="glass-card large-display-card mission-glow" on:click=move |_| set_active_tab.set(Tab::Mission)>
                            <div class="card-header">
                                <span class="card-title">"Core Mission"</span>
                                <span class="card-subtitle">"Tap to edit"</span>
                            </div>
                            <div class="card-content">
                                {move || if vision_data.get().mission.is_empty() {
                                    "No core mission set. Tap to define your daily engine.".to_string()
                                } else {
                                    vision_data.get().mission
                                }}
                            </div>
                        </div>
                    }.into_view(),

                    Tab::Mission => view! {
                        <div class="glass-card">
                            <div class="card-header">
                                <span class="card-title">"Edit Mission Statement"</span>
                            </div>
                            <div class="form-group">
                                <label class="form-label">"Core Mission"</label>
                                <textarea class="form-input"
                                    prop:value=edit_mission
                                    on:input=move |ev| set_edit_mission.set(event_target_value(&ev))
                                    placeholder="What is your mission statement? What builds your focus daily?"
                                />
                            </div>
                            <div style="display: flex; gap: 10px;">
                                <button class="btn" on:click=handle_save_vision>"Save Mission"</button>
                                <button class="btn btn-secondary" on:click=move |_| set_active_tab.set(Tab::Home)>"Cancel"</button>
                            </div>
                        </div>
                    }.into_view(),

                    Tab::Vision => view! {
                        <div class="glass-card">
                            <div class="card-header">
                                <span class="card-title">"Edit Vision Statement"</span>
                            </div>
                            <div class="form-group">
                                <label class="form-label">"Personal Vision Statement"</label>
                                <textarea class="form-input"
                                    prop:value=edit_vision
                                    on:input=move |ev| set_edit_vision.set(event_target_value(&ev))
                                    placeholder="What is your long-term personal vision? What inspires you?"
                                />
                            </div>
                            <div style="display: flex; gap: 10px;">
                                <button class="btn" on:click=handle_save_vision>"Save Vision"</button>
                                <button class="btn btn-secondary" on:click=move |_| set_active_tab.set(Tab::Home)>"Cancel"</button>
                            </div>
                        </div>
                    }.into_view(),

                    Tab::Goals => view! {
                        <div class="glass-card">
                            <div class="card-header">
                                <span class="card-title">"Configure Goals"</span>
                            </div>
                            
                            <div class="form-group">
                                <label class="form-label">"Yearly Goal"</label>
                                <input type="text" class="form-input"
                                    prop:value=edit_yearly
                                    on:input=move |ev| set_edit_yearly.set(event_target_value(&ev))
                                    placeholder="e.g. Master Rust, build premium apps"
                                />
                            </div>

                            <div class="form-group">
                                <label class="form-label">"Quarter Goal"</label>
                                <input type="text" class="form-input"
                                    prop:value=edit_quarterly
                                    on:input=move |ev| set_edit_quarterly.set(event_target_value(&ev))
                                    placeholder="e.g. Ship 3 personal projects"
                                />
                            </div>

                            <div class="form-group">
                                <label class="form-label">"Monthly Goal"</label>
                                <input type="text" class="form-input"
                                    prop:value=edit_monthly
                                    on:input=move |ev| set_edit_monthly.set(event_target_value(&ev))
                                    placeholder="e.g. Finish Axum api and Leptos client"
                                />
                            </div>

                            <div class="form-group">
                                <label class="form-label">"Weekly Goal"</label>
                                <input type="text" class="form-input"
                                    prop:value=edit_weekly
                                    on:input=move |ev| set_edit_weekly.set(event_target_value(&ev))
                                    placeholder="e.g. Set up database and model files"
                                />
                            </div>

                            <div style="display: flex; gap: 10px; margin-top: 10px;">
                                <button class="btn" on:click=handle_save_goals>"Save Goals"</button>
                                <button class="btn btn-secondary" on:click=move |_| set_active_tab.set(Tab::Home)>"Cancel"</button>
                            </div>
                        </div>
                    }.into_view(),

                    Tab::Reminder => view! {
                        <div class="glass-card">
                            <div class="card-header">
                                <span class="card-title">"Focus Reminders"</span>
                                <span class="card-subtitle">"Daily triggers"</span>
                            </div>
                            
                            <form on:submit=handle_add_reminder style="display: flex; gap: 10px; margin-top: 10px;">
                                <input type="text" class="form-input" style="flex: 1; margin-bottom: 0;"
                                    prop:value=new_reminder_text
                                    on:input=move |ev| set_new_reminder_text.set(event_target_value(&ev))
                                    placeholder="Add a new daily focus trigger..."
                                />
                                <button class="btn" type="submit">"Add"</button>
                            </form>

                            <div class="reminder-list">
                                {move || {
                                    let list = reminders.get();
                                    if list.is_empty() {
                                        view! {
                                            <div class="empty-state">
                                                <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                    <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                                                    <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                                                </svg>
                                                <span>"No reminders active. Add one above!"</span>
                                            </div>
                                        }.into_view()
                                    } else {
                                        list.into_iter().map(|item| {
                                            let it = item.clone();
                                            let toggle_it = item.clone();
                                            let delete_id = item.id.unwrap_or(0);
                                            view! {
                                                <div class=move || if it.is_completed { "reminder-item completed" } else { "reminder-item" }>
                                                    <div class="reminder-checkbox" on:click=move |_| handle_toggle_reminder(toggle_it.clone())>
                                                        {move || if it.is_completed {
                                                            view! {
                                                                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" style="color: white;">
                                                                    <polyline points="20 6 9 17 4 12"></polyline>
                                                                </svg>
                                                            }.into_view()
                                                        } else {
                                                            view! { "" }.into_view()
                                                        }}
                                                    </div>
                                                    <span class="reminder-text">{item.text.clone()}</span>
                                                    <button class="btn-delete" on:click=move |_| handle_delete_reminder(delete_id)>
                                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                            <polyline points="3 6 5 6 21 6"></polyline>
                                                            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                                                            <line x1="10" y1="11" x2="10" y2="17"></line>
                                                            <line x1="14" y1="11" x2="14" y2="17"></line>
                                                        </svg>
                                                    </button>
                                                </div>
                                            }.into_view()
                                        }).collect_view()
                                    }
                                }}
                            </div>
                        </div>
                    }.into_view()
                }}
            </main>

            // Bottom Navigation
            <nav class="bottom-nav">
                <button class=move || if active_tab.get() == Tab::Home { "nav-item active" } else { "nav-item" }
                    on:click=move |_| set_active_tab.set(Tab::Home)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                        <polyline points="9 22 9 12 15 12 15 22"></polyline>
                    </svg>
                    <span>"Home"</span>
                </button>
                
                <button class=move || if active_tab.get() == Tab::Mission { "nav-item active" } else { "nav-item" }
                    on:click=move |_| set_active_tab.set(Tab::Mission)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"></circle>
                        <circle cx="12" cy="12" r="6"></circle>
                        <circle cx="12" cy="12" r="2"></circle>
                    </svg>
                    <span>"Mission"</span>
                </button>

                <button class=move || if active_tab.get() == Tab::Vision { "nav-item active" } else { "nav-item" }
                    on:click=move |_| set_active_tab.set(Tab::Vision)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
                        <circle cx="12" cy="12" r="3"></circle>
                    </svg>
                    <span>"Vision"</span>
                </button>

                <button class=move || if active_tab.get() == Tab::Goals { "nav-item active" } else { "nav-item" }
                    on:click=move |_| set_active_tab.set(Tab::Goals)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"></path>
                        <path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"></path>
                        <path d="M4 22h16"></path>
                        <path d="M10 14.66V17c0 .55-.45 1-1 1H4v2h16v-2h-5c-.55 0-1-.45-1-1v-2.34"></path>
                        <path d="M12 2a4.7 4.7 0 0 1 4 7c-.55 1.5-2 4-4 4s-3.5-2.5-4-4a4.7 4.7 0 0 1 4-7z"></path>
                    </svg>
                    <span>"Goals"</span>
                </button>

                <button class=move || if active_tab.get() == Tab::Reminder { "nav-item active" } else { "nav-item" }
                    on:click=move |_| set_active_tab.set(Tab::Reminder)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
                        <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
                    </svg>
                    <span>"Reminder"</span>
                </button>
            </nav>
        </div>
    }
}
