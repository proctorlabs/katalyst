use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Authentication {
    Anonymous,
    Authenticated { claims: HashMap<String, Vec<String>> },
}

impl Authentication {
    pub fn add_claim(&mut self, claim_type: String, claim_value: String) {
        if let Authentication::Authenticated { claims } = self {
            if let Some(claim) = claims.get_mut(&claim_type) {
                claim.push(claim_value);
            } else {
                claims.insert(claim_type, vec![claim_value]);
            }
        }
    }

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
