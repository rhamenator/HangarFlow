use hangar_flow::{MaintenanceTask, TaskStatus, WorkOrder};

fn main() {
    let order = WorkOrder {
        number: "WO-DEMO".into(),
        aircraft_registration: "N-DEMO".into(),
        tasks: vec![MaintenanceTask {
            description: "Annual inspection".into(),
            status: TaskStatus::Approved,
            labor_minutes: 480,
            labor_rate_cents_per_hour: 14_000,
            parts_cost_cents: 35_000,
        }],
    };
    println!(
        "{} quote: ${:.2}",
        order.number,
        order.quoted_total_cents() as f64 / 100.0
    );
}
