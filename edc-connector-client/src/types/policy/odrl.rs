#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::types::policy::{AtomicConstraint, Constraint, Operator, Policy, PolicyKind};

    #[test]
    fn should_deserialize_odrl() {
        let json = json!({
            "@context": "http://www.w3.org/ns/odrl.jsonld",
            "@type": "Set",
            "uid": "https://w3c.github.io/odrl/bp/examples/3",
            "permission": [{
                "target": "http://example.com/asset:9898.movie",
                "action": "display",
                "constraint": [{
                   "leftOperand": "spatial",
                   "operator": "eq",
                   "rightOperand":  "https://www.wikidata.org/resource/Q183",
                     "dct:comment": "i.e Germany"
               }]
            }]
        });

        let policy = serde_json::from_value::<Policy>(json).unwrap();

        assert_eq!(policy.kind(), &PolicyKind::Set);
        assert_eq!(policy.permissions().len(), 1);

        let permission = &policy.permissions[0];

        assert_eq!(permission.action().id(), "display");
        assert_eq!(permission.constraints().len(), 1);

        let constraint = &permission.constraints()[0];

        assert_eq!(
            constraint,
            &Constraint::Atomic(AtomicConstraint::new(
                "spatial",
                "eq",
                "https://www.wikidata.org/resource/Q183"
            ))
        );
    }

    #[test]
    fn should_deserialize_edc_prefixed() {
        let json = json!({
            "@id": "b3e9255b-14c9-4a2b-a439-50b9382b81b1",
            "@type": "odrl:Set",
            "odrl:permission": {
                "odrl:action": {
                    "@id": "http://www.w3.org/ns/odrl/2/use"
                },
                "odrl:constraint": {
                    "odrl:leftOperand": {
                      "@id": "https://w3id.org/edc/v0.0.1/ns/foo"
                    },
                    "odrl:operator": {
                        "@id": "odrl:eq"
                    },
                    "odrl:rightOperand": "bar"
                }
            },
            "odrl:prohibition": [],
            "odrl:obligation": []
        });

        let policy = serde_json::from_value::<Policy>(json).unwrap();

        assert_eq!(policy.kind(), &PolicyKind::Set);
        assert_eq!(policy.permissions().len(), 1);

        let permission = &policy.permissions[0];

        assert_eq!(permission.action().id(), "http://www.w3.org/ns/odrl/2/use");
        assert_eq!(permission.constraints().len(), 1);

        let constraint = &permission.constraints()[0];

        assert_eq!(
            constraint,
            &Constraint::Atomic(AtomicConstraint::new_with_operator(
                "https://w3id.org/edc/v0.0.1/ns/foo",
                Operator::id("odrl:eq"),
                "bar"
            ))
        );
    }
}
