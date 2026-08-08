#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Quoted,
    Approved,
    InProgress,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaintenanceTask {
    pub description: String,
    pub status: TaskStatus,
    pub labor_minutes: u32,
    pub labor_rate_cents_per_hour: u64,
    pub parts_cost_cents: u64,
}

impl MaintenanceTask {
    pub fn extended_cost_cents(&self) -> u64 {
        self.parts_cost_cents
            + (u64::from(self.labor_minutes) * self.labor_rate_cents_per_hour).div_ceil(60)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkOrder {
    pub number: String,
    pub aircraft_registration: String,
    pub tasks: Vec<MaintenanceTask>,
}

impl WorkOrder {
    pub fn quoted_total_cents(&self) -> u64 {
        self.tasks
            .iter()
            .map(MaintenanceTask::extended_cost_cents)
            .sum()
    }

    pub fn is_ready_to_invoice(&self) -> bool {
        !self.tasks.is_empty()
            && self
                .tasks
                .iter()
                .all(|task| task.status == TaskStatus::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_uses_parts_and_minute_precision_labor() {
        let order = WorkOrder {
            number: "WO-1".into(),
            aircraft_registration: "N-DEMO".into(),
            tasks: vec![MaintenanceTask {
                description: "Inspection".into(),
                status: TaskStatus::Complete,
                labor_minutes: 90,
                labor_rate_cents_per_hour: 12_000,
                parts_cost_cents: 5_000,
            }],
        };
        assert_eq!(order.quoted_total_cents(), 23_000);
        assert!(order.is_ready_to_invoice());
    }
}
