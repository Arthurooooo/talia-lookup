// Bubble Data API client — miroir de BubbleClient.swift côté Mac.
//
// PIÈGE serde : Bubble retourne `_id`, `fullName`, `student` etc. — des noms
// qui ne sont pas en camelCase strict. On veut désérialiser ces noms-là côté
// entrée (`alias`), mais sérialiser des noms camelCase propres côté sortie
// (vers le front TypeScript). D'où `alias` au lieu de `rename`.

use reqwest::Client;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://eleve.talia.fr/api/1.1/obj";

// ---------- Models ----------

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BubbleUser {
    #[serde(alias = "_id")]
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(alias = "fullName")]
    pub api_full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub student_status: Option<String>,
    #[serde(alias = "status")]
    pub lead_status: Option<String>,
    pub etat_lead: Option<String>,
    pub lead_source: Option<String>,
    pub lead_rating: Option<f64>,
    pub role: Option<String>,
    pub monday_id: Option<String>,
    pub birth_date: Option<String>,
    pub became_student_date: Option<String>,
    pub activated_date: Option<String>,
    pub last_login_date: Option<String>,
    #[serde(alias = "CurrentSessionEndDate")]
    pub current_training_end_date: Option<String>,
    pub profile_picture: Option<String>,
    pub gender: Option<String>,
    pub nationality: Option<String>,
    #[serde(alias = "userAccountManager")]
    pub account_manager_id: Option<String>,
    #[serde(alias = "lastcall")]
    pub last_call_date: Option<String>,
    pub follow_up_call_date: Option<String>,
    pub authentication: Option<UserAuth>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAuth {
    pub email: Option<UserAuthEmail>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAuthEmail {
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BubbleContract {
    #[serde(alias = "_id")]
    pub id: String,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_date: Option<String>,
    #[serde(alias = "1MonthVerificationDate")]
    pub one_month_verification_date: Option<String>,
    pub termination_date: Option<String>,
    pub termination_reason: Option<String>,
    pub is_terminated: Option<bool>,
    pub is_cancelled: Option<bool>,
    pub is_termination_occurred: Option<bool>,
    pub is_termination_accepted: Option<bool>,
    pub is_termination_signed: Option<bool>,
    pub is_termination_submitted_to_opco: Option<bool>,
    #[serde(alias = "ContractAmount")]
    pub contract_amount: Option<f64>,
    pub training_amount: Option<f64>,
    pub number_of_hours_in_elearning: Option<f64>,
    pub number_of_hours_in_virtual_classroom: Option<f64>,
    #[serde(alias = "student")]
    pub student_id: Option<String>,
    pub class_id: Option<String>,
    pub monday_id: Option<String>,
    #[serde(alias = "training")]
    pub training_id: Option<String>,
    #[serde(alias = "company")]
    pub company_id: Option<String>,
    #[serde(alias = "tutor")]
    pub tutor_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BubbleSession {
    #[serde(alias = "_id")]
    pub id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(alias = "student")]
    pub student_id: Option<String>,
    #[serde(alias = "training")]
    pub training_id: Option<String>,
    #[serde(alias = "mainTeacher")]
    pub main_teacher_id: Option<String>,
    #[serde(alias = "pedagogicalReferent")]
    pub pedagogical_referent_id: Option<String>,
    pub school_day: Option<String>,
    pub trimester: Option<String>,
    pub year: Option<String>,
    #[serde(alias = "type")]
    pub session_type: Option<String>,
    pub required_hours: Option<String>,
    pub regular_tickets: Option<f64>,
    pub remaining_regular_tickets: Option<f64>,
    pub exam_tickets: Option<f64>,
    pub remaining_exam_tickets: Option<f64>,
    pub absence_warnings: Option<f64>,
    pub is_in_class: Option<bool>,
    pub disabled_date: Option<String>,
    pub exam_date: Option<String>,
    pub teacher_assignment_date: Option<String>,
    #[serde(alias = "tutorCompany")]
    pub tutor_company_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BubbleMeeting {
    #[serde(alias = "_id")]
    pub id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub meeting_type: Option<String>,
    pub meeting_status: Option<String>,
    #[serde(alias = "attendees")]
    pub attendee_ids: Option<Vec<String>>,
    #[serde(alias = "organizer")]
    pub organizer_id: Option<String>,
    #[serde(alias = "userAccountManager")]
    pub account_manager_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BubbleClass {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub school_day: Option<String>,
    #[serde(alias = "teachers")]
    pub teacher_ids: Option<Vec<String>>,
    #[serde(alias = "classManager")]
    pub class_manager_id: Option<String>,
    pub exam_date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BubbleResponse<T> {
    response: BubbleResponseInner<T>,
}
#[derive(Debug, Deserialize)]
struct BubbleResponseInner<T> {
    results: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct BubbleSingle<T> {
    response: T,
}

// ---------- Client ----------

fn http() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap()
}

async fn get_list<T: serde::de::DeserializeOwned>(
    token: &str,
    obj: &str,
    constraints_json: &str,
    sort_field: Option<&str>,
    limit: u32,
) -> Result<Vec<T>, String> {
    let mut url = format!(
        "{}/{}?constraints={}&limit={}",
        BASE_URL,
        obj,
        urlencoding::encode(constraints_json),
        limit
    );
    if let Some(sf) = sort_field {
        url.push_str(&format!("&sort_field={}&descending=true", urlencoding::encode(sf)));
    }
    let resp = http()
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Réseau : {}", e))?;
    let status = resp.status();
    let body = resp.text().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!("HTTP {} — {}", status.as_u16(), &body.chars().take(200).collect::<String>()));
    }
    let parsed: BubbleResponse<T> = serde_json::from_str(&body)
        .map_err(|e| format!("JSON: {} / body: {}", e, &body.chars().take(200).collect::<String>()))?;
    Ok(parsed.response.results)
}

async fn get_one<T: serde::de::DeserializeOwned>(
    token: &str,
    obj: &str,
    id: &str,
) -> Result<Option<T>, String> {
    let url = format!("{}/{}/{}", BASE_URL, obj, id);
    let resp = http().get(&url).bearer_auth(token).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() { return Ok(None); }
    let body = resp.text().await.map_err(|e| e.to_string())?;
    let parsed: BubbleSingle<T> = serde_json::from_str(&body).map_err(|e| e.to_string())?;
    Ok(Some(parsed.response))
}

// ---------- Search ----------

pub async fn search_users(token: &str, query: &str) -> Result<Vec<BubbleUser>, String> {
    let q = query.trim();
    if q.is_empty() { return Ok(vec![]); }
    let c = format!(
        r#"[{{"key":"search_field_text","constraint_type":"text contains","value":{}}}]"#,
        serde_json::to_string(q).unwrap()
    );
    let primary: Vec<BubbleUser> = get_list(token, "user", &c, None, 10).await?;
    if !primary.is_empty() { return Ok(primary); }

    let tokens: Vec<&str> = q.split_whitespace().filter(|t| t.len() >= 2).collect();
    if tokens.is_empty() { return Ok(vec![]); }

    let mut combined: Vec<BubbleUser> = vec![];
    let mut seen = std::collections::HashSet::new();
    for field in &["first_name_text", "last_name_text"] {
        let constraints: Vec<String> = tokens
            .iter()
            .map(|t| format!(
                r#"{{"key":"{}","constraint_type":"text contains","value":{}}}"#,
                field, serde_json::to_string(t).unwrap()
            ))
            .collect();
        let cj = format!("[{}]", constraints.join(","));
        if let Ok(res) = get_list::<BubbleUser>(token, "user", &cj, None, 10).await {
            for u in res {
                if seen.insert(u.id.clone()) { combined.push(u); }
            }
        }
    }
    if tokens.len() >= 2 {
        let cj = format!(
            r#"[{{"key":"first_name_text","constraint_type":"text contains","value":{}}},{{"key":"last_name_text","constraint_type":"text contains","value":{}}}]"#,
            serde_json::to_string(tokens[0]).unwrap(),
            serde_json::to_string(tokens[tokens.len()-1]).unwrap()
        );
        if let Ok(res) = get_list::<BubbleUser>(token, "user", &cj, None, 10).await {
            for u in res {
                if seen.insert(u.id.clone()) { combined.push(u); }
            }
        }
    }
    Ok(combined)
}

pub async fn contracts_for(token: &str, user_id: &str) -> Result<Vec<BubbleContract>, String> {
    let c = format!(
        r#"[{{"key":"user_user","constraint_type":"equals","value":{}}}]"#,
        serde_json::to_string(user_id).unwrap()
    );
    get_list(token, "contract", &c, Some("startdate_date"), 100).await
}

pub async fn sessions_for(token: &str, user_id: &str) -> Result<Vec<BubbleSession>, String> {
    let c = format!(
        r#"[{{"key":"user_user","constraint_type":"equals","value":{}}}]"#,
        serde_json::to_string(user_id).unwrap()
    );
    get_list(token, "session", &c, Some("start_date_date"), 100).await
}

pub async fn meetings_for(token: &str, user_id: &str) -> Result<Vec<BubbleMeeting>, String> {
    let c = format!(
        r#"[{{"key":"attendees_list_user","constraint_type":"contains","value":{}}}]"#,
        serde_json::to_string(user_id).unwrap()
    );
    get_list(token, "meeting", &c, Some("startdate_date"), 50).await
}

pub async fn get_user(token: &str, id: &str) -> Result<Option<BubbleUser>, String> {
    get_one(token, "user", id).await
}

pub async fn get_class(token: &str, id: &str) -> Result<Option<BubbleClass>, String> {
    get_one(token, "class", id).await
}
