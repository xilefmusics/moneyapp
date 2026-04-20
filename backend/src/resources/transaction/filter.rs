use chrono::{DateTime, Local};

#[derive(Default, Clone)]
pub struct Filter<'a> {
    start: Option<DateTime<Local>>,
    end: Option<DateTime<Local>>,
    pod: Option<&'a str>,
    debt: Option<&'a str>,
    budget: Option<&'a str>,
    inbudget: Option<&'a str>,
    ttype: Option<&'a str>,
}

impl<'a> Filter<'a> {
    pub fn new(
        start: Option<DateTime<Local>>,
        end: Option<DateTime<Local>>,
        pod: Option<&'a str>,
        debt: Option<&'a str>,
        budget: Option<&'a str>,
        inbudget: Option<&'a str>,
        ttype: Option<&'a str>,
    ) -> Self {
        Self {
            start,
            end,
            pod,
            debt,
            budget,
            inbudget,
            ttype,
        }
    }

    pub fn start_option(self, start: Option<DateTime<Local>>) -> Self {
        let mut result = self.clone();
        result.start = start;
        result
    }

    /// SurrealQL `WHERE` fragments (without leading `AND`), same semantics as legacy fancy_surreal filters.
    pub fn conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();

        if let Some(start) = self.start {
            conditions.push(format!("content.date >= \"{}\"", start));
        }

        if let Some(end) = self.end {
            conditions.push(format!("content.date < \"{}\"", end));
        }

        if let Some(pod) = self.pod {
            conditions.push(format!(
                "(content.sender == \"{}\" OR content.receiver == \"{}\")",
                pod, pod
            ));
        }
        if let Some(debt) = self.debt {
            conditions.push(format!("content.debts[\"{}\"]", debt));
        }
        if let Some(budget) = self.budget {
            conditions.push(format!("content.budgets[\"{}\"]", budget));
        }
        if let Some(inbudget) = self.inbudget {
            conditions.push(format!("content.inbudgets[\"{}\"]", inbudget));
        }
        if let Some(ttype) = self.ttype {
            conditions.push(format!("content.type == \"{}\"", ttype));
        }

        conditions
    }
}
