use std::{collections::BTreeMap, fmt, ops::RangeBounds};

use ledger::scan_state::scan_state::{transaction_snark::OneOrTwo, AvailableJobMessage};
use redux::Timestamp;
use serde::{Deserialize, Serialize};
use shared::snark::Snark;
use shared::snark_job_id::SnarkJobId;

use crate::p2p::{channels::snark_job_commitment::SnarkJobCommitment, PeerId};

use super::SnarkPoolConfig;

#[derive(Clone)]
pub struct SnarkPoolState {
    config: SnarkPoolConfig,
    counter: u64,
    list: BTreeMap<u64, JobState>,
    by_ledger_hash_index: BTreeMap<SnarkJobId, u64>,
    pub(super) last_check_timeouts: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobState {
    pub time: Timestamp,
    pub id: SnarkJobId,
    pub job: OneOrTwo<AvailableJobMessage>,
    pub commitment: Option<JobCommitment>,
    pub snark: Option<SnarkWork>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobCommitment {
    pub commitment: SnarkJobCommitment,
    pub received_t: Timestamp,
    pub sender: PeerId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SnarkWork {
    pub work: Snark,
    pub received_t: Timestamp,
    pub sender: PeerId,
}

impl SnarkPoolState {
    pub fn new(config: SnarkPoolConfig) -> Self {
        Self {
            config,
            counter: 0,
            list: Default::default(),
            by_ledger_hash_index: Default::default(),
            last_check_timeouts: Timestamp::ZERO,
        }
    }

    pub fn last_index(&self) -> u64 {
        self.list.last_key_value().map_or(0, |(k, _)| *k)
    }

    pub fn contains(&self, id: &SnarkJobId) -> bool {
        self.by_ledger_hash_index
            .get(id)
            .map_or(false, |i| self.list.contains_key(i))
    }

    pub fn get(&self, id: &SnarkJobId) -> Option<&JobState> {
        self.by_ledger_hash_index
            .get(id)
            .and_then(|i| self.list.get(i))
    }

    pub fn insert(&mut self, job: JobState) {
        let id = job.id.clone();
        self.list.insert(self.counter, job);
        self.by_ledger_hash_index.insert(id, self.counter);
        self.counter += 1;
    }

    pub fn remove(&mut self, id: &SnarkJobId) -> Option<JobState> {
        let index = self.by_ledger_hash_index.remove(id)?;
        self.list.remove(&index)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&SnarkJobId) -> bool,
    {
        let list = &mut self.list;
        self.by_ledger_hash_index.retain(|id, index| {
            let keep = f(id);
            if !keep {
                list.remove(index);
            }
            keep
        });
    }

    pub fn range<'a, R>(
        &'a self,
        range: R,
    ) -> impl 'a + DoubleEndedIterator<Item = (u64, &'a JobState)>
    where
        R: RangeBounds<u64>,
    {
        self.list.range(range).map(|(k, v)| (*k, v))
    }

    pub fn should_create_commitment(&self, job_id: &SnarkJobId) -> bool {
        self.get(job_id).map_or(false, |s| s.is_available())
    }

    pub fn is_commitment_timed_out(&self, id: &SnarkJobId, time_now: Timestamp) -> bool {
        self.by_ledger_hash_index.get(id).map_or(false, |i| {
            self.is_commitment_timed_out_by_index(i, time_now)
        })
    }

    pub fn is_commitment_timed_out_by_index(&self, index: &u64, time_now: Timestamp) -> bool {
        self.list
            .get(index)
            .and_then(|v| v.commitment.as_ref())
            .map(|v| v.commitment.timestamp())
            .and_then(|commitment_time| time_now.checked_sub(commitment_time))
            .map_or(false, |dur| dur >= self.config.commitment_timeout)
    }

    pub fn timed_out_commitments_iter(
        &self,
        time_now: Timestamp,
    ) -> impl Iterator<Item = &SnarkJobId> {
        self.by_ledger_hash_index
            .iter()
            .filter(move |(_, index)| self.is_commitment_timed_out_by_index(index, time_now))
            .map(|(id, _)| id)
    }

    pub fn available_jobs_iter<'a>(&'a self) -> impl 'a + Iterator<Item = &'a JobState> {
        self.list
            .iter()
            .map(|(_, job)| job)
            .filter(|job| job.is_available())
    }
}

impl fmt::Debug for SnarkPoolState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JobCommitments")
            .field("counter", &self.counter)
            .field("len", &self.list.len())
            .finish()
    }
}

impl JobState {
    pub fn is_available(&self) -> bool {
        self.commitment.is_none() && self.snark.is_none()
    }
}

mod ser {
    use super::*;
    use serde::ser::SerializeStruct;

    #[derive(Serialize, Deserialize)]
    struct SnarkPool {
        config: SnarkPoolConfig,
        counter: u64,
        list: BTreeMap<u64, JobState>,
        last_check_timeouts: Timestamp,
    }

    impl Serialize for super::SnarkPoolState {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let mut s = serializer.serialize_struct("SnarkPool", 4)?;
            s.serialize_field("config", &self.config)?;
            s.serialize_field("counter", &self.counter)?;
            s.serialize_field("list", &self.list)?;
            s.serialize_field("last_check_timeouts", &self.last_check_timeouts)?;
            s.end()
        }
    }
    impl<'de> Deserialize<'de> for super::SnarkPoolState {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let v = SnarkPool::deserialize(deserializer)?;
            let by_ledger_hash_index = v.list.iter().map(|(k, v)| (v.id.clone(), *k)).collect();
            Ok(Self {
                config: v.config,
                counter: v.counter,
                list: v.list,
                by_ledger_hash_index,
                last_check_timeouts: v.last_check_timeouts,
            })
        }
    }
}