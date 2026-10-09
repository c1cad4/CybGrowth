//! Evidence-linked contributions and a simple non-financial activity score.
//! Evidence validity is checked by a separate review system.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Contribution {
    pub id: String,
    pub contributor: String,
    pub mission_id: String,
    pub evidence_id: String,
    pub approved: bool,
}

#[derive(Default)]
pub struct GrowthLedger {
    contributions: BTreeMap<String, Contribution>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GrowthError {
    MissingEvidence,
    MissingIdentity,
    DuplicateContribution,
}

impl GrowthLedger {
    pub fn submit(&mut self, contribution: Contribution) -> Result<(), GrowthError> {
        if contribution.evidence_id.trim().is_empty() {
            return Err(GrowthError::MissingEvidence);
        }
        if contribution.id.trim().is_empty()
            || contribution.contributor.trim().is_empty()
            || contribution.mission_id.trim().is_empty()
        {
            return Err(GrowthError::MissingIdentity);
        }
        if self.contributions.contains_key(&contribution.id) {
            return Err(GrowthError::DuplicateContribution);
        }
        self.contributions
            .insert(contribution.id.clone(), contribution);
        Ok(())
    }

    pub fn completed_missions(&self, contributor: &str) -> usize {
        self.contributions
            .values()
            .filter(|c| c.contributor == contributor && c.approved)
            .map(|c| c.mission_id.as_str())
            .collect::<BTreeSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn contribution(id: &str, mission: &str) -> Contribution {
        Contribution {
            id: id.into(),
            contributor: "human-a".into(),
            mission_id: mission.into(),
            evidence_id: "report-1".into(),
            approved: true,
        }
    }

    #[test]
    fn counts_distinct_approved_missions() {
        let mut ledger = GrowthLedger::default();
        ledger.submit(contribution("1", "m1")).unwrap();
        ledger.submit(contribution("2", "m1")).unwrap();
        assert_eq!(ledger.completed_missions("human-a"), 1);
    }

    #[test]
    fn rejects_duplicate_contribution() {
        let mut ledger = GrowthLedger::default();
        ledger.submit(contribution("1", "m1")).unwrap();
        assert_eq!(
            ledger.submit(contribution("1", "m2")),
            Err(GrowthError::DuplicateContribution)
        );
    }

    #[test]
    fn requires_evidence() {
        let mut ledger = GrowthLedger::default();
        let mut c = contribution("1", "m1");
        c.evidence_id.clear();
        assert_eq!(ledger.submit(c), Err(GrowthError::MissingEvidence));
    }
}
