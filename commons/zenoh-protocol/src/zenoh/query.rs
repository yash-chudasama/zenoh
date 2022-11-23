use rand::seq::SliceRandom;

//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use crate::core::{ConsolidationMode, QueryTarget, WireExpr, ZInt};

/// # Query message
///
/// ```text
///  7 6 5 4 3 2 1 0
/// +-+-+-+-+-+-+-+-+
/// |K|X|T|  QUERY  |
/// +-+-+-+---------+
/// ~    KeyExpr     ~ if K==1 then key_expr has suffix
/// +---------------+
/// ~selector_params~
/// +---------------+
/// ~      qid      ~
/// +---------------+
/// ~     target    ~ if T==1
/// +---------------+
/// ~ consolidation ~
/// +---------------+
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pub key: WireExpr<'static>,
    pub parameters: String,
    pub qid: ZInt,
    pub target: Option<QueryTarget>,
    pub consolidation: ConsolidationMode,
}

// Functions mainly used for testing
impl Query {
    #[doc(hidden)]
    pub fn rand() -> Self {
        use rand::{
            distributions::{Alphanumeric, DistString},
            Rng,
        };

        const MIN: usize = 2;
        const MAX: usize = 16;

        let mut rng = rand::thread_rng();

        let key = WireExpr::rand();

        let parameters = if rng.gen_bool(0.5) {
            let len = rng.gen_range(MIN..MAX);
            Alphanumeric.sample_string(&mut rng, len)
        } else {
            String::new()
        };

        let qid: ZInt = rng.gen();

        let target = if rng.gen_bool(0.5) {
            let t = [
                QueryTarget::All,
                QueryTarget::AllComplete,
                QueryTarget::BestMatching,
                #[cfg(feature = "complete_n")]
                QueryTarget::Complete(rng.gen()),
            ];
            let t = t.choose(&mut rng).unwrap();
            Some(*t)
        } else {
            None
        };
        let consolidation = *[
            ConsolidationMode::Latest,
            ConsolidationMode::Monotonic,
            ConsolidationMode::None,
        ]
        .choose(&mut rng)
        .unwrap();

        Self {
            key,
            parameters,
            qid,
            target,
            consolidation,
        }
    }
}
