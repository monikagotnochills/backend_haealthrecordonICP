use ic_cdk_macros::*;
use ic_cdk::println;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct MedicalRecord {
    history: String,
    test_results: String,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
struct Appointment {
    datetime: String,
}

thread_local! {
    static PATIENTS: RefCell<HashMap<String, MedicalRecord>> = RefCell::new(HashMap::new());
    static APPOINTMENTS: RefCell<HashMap<String, Vec<Appointment>>> = RefCell::new(HashMap::new());
}

// View functions
#[update]
fn view_medical_history(patient_id: String) -> String {
    PATIENTS.with(|p| {
        p.borrow().get(&patient_id)
            .map(|r| r.history.clone())
            .unwrap_or("No record found.".to_string())
    })
}

#[update]
fn get_test_results(patient_id: String) -> String {
    PATIENTS.with(|p| {
        p.borrow().get(&patient_id)
            .map(|r| r.test_results.clone())
            .unwrap_or("No results available.".to_string())
    })
}

// Write functions
#[update]
fn update_medical_record(patient_id: String, history: String, test_results: String) -> bool {
    PATIENTS.with(|p| {
        p.borrow_mut().insert(patient_id, MedicalRecord { history, test_results });
    });
    true
}

#[update]
fn schedule_appointment(patient_id: String, datetime: String) -> bool {
    APPOINTMENTS.with(|a| {
        let mut map = a.borrow_mut();
        let entry = map.entry(patient_id).or_insert(vec![]);
        entry.push(Appointment { datetime });
    });
    true
}

#[update]
fn get_appointments(patient_id: String) -> Vec<String> {
    APPOINTMENTS.with(|a| {
        a.borrow()
            .get(&patient_id)
            .map(|v| v.iter().map(|appt| appt.datetime.clone()).collect())
            .unwrap_or_default()
    })
}

// Doctor-side analytics
#[update]
fn patient_management_dashboard() -> String {
    format!("Total patients: {}", PATIENTS.with(|p| p.borrow().len()))
}

#[update]
fn clinical_analytics() -> String {
    let total_appointments = APPOINTMENTS.with(|a| {
        a.borrow().values().map(|v| v.len()).sum::<usize>()
    });
    format!("Appointments scheduled: {}", total_appointments)
}
