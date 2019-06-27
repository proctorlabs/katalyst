use std::collections::HashMap;

/// Authentication detail for this context
#[derive(Debug, Clone)]
pub enum Authentication {
    /// An anonymous request (not authenticated)
    Anonymous,
    /// An authenticated request
    Authenticated {
        /// The claims associated with this request
        claims: HashMap<String, Vec<String>>,
    },
}

impl Authentication {
    /// Add a claim to this authentication context. This has no effect if the current
    /// authentication type is anonymous.
    pub fn add_claim(&mut self, claim_type: String, claim_value: String) {
        if let Authentication::Authenticated { claims } = self {
            if let Some(claim) = claims.get_mut(&claim_type) {
                claim.push(claim_value);
            } else {
                claims.insert(claim_type, vec![claim_value]);
            }
        }
    }

    /// Retrieve a claim from this authentication context.
    pub fn get_claim(&self, claim_type: String) -> String {
        if let Authentication::Authenticated { claims } = self {
            match claims.get(&claim_type) {
                Some(c) => c[0].to_string(),
                None => String::default(),
            }
        } else {
            String::default()
        }
    }
}
