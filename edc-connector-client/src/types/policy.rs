mod odrl;

use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};

use crate::{BuilderError, ConversionError};

use super::properties::{FromValue, Properties, PropertyValue, ToValue};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PolicyDefinition {
    #[serde(rename = "@id")]
    id: String,
    policy: Policy,
    #[serde(default)]
    private_properties: Properties,
}

impl PolicyDefinition {
    pub fn builder() -> PolicyDefinitionBuilder {
        PolicyDefinitionBuilder::default()
    }

    pub fn policy(&self) -> &Policy {
        &self.policy
    }

    pub fn private_property<T>(&self, property: &str) -> Result<Option<T>, ConversionError>
    where
        T: FromValue,
    {
        self.private_properties.get(property)
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Default)]
pub struct PolicyDefinitionBuilder {
    id: Option<String>,
    policy: Option<Policy>,
    private_properties: Properties,
}

impl PolicyDefinitionBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn policy(mut self, policy: Policy) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.private_properties.set(property, value);
        self
    }

    pub fn build(self) -> Result<PolicyDefinition, BuilderError> {
        Ok(PolicyDefinition {
            id: self
                .id
                .ok_or_else(|| BuilderError::missing_property("id"))?,
            policy: self
                .policy
                .ok_or_else(|| BuilderError::missing_property("policy"))?,

            private_properties: self.private_properties,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPolicyDefinition {
    #[serde(rename = "@id")]
    id: Option<String>,
    policy: Policy,
    #[serde(default)]
    private_properties: Properties,
}

impl NewPolicyDefinition {
    pub fn builder() -> NewPolicyDefinitionBuilder {
        NewPolicyDefinitionBuilder(NewPolicyDefinition::default())
    }
}

#[derive(Default)]
pub struct NewPolicyDefinitionBuilder(NewPolicyDefinition);

impl NewPolicyDefinitionBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.0.id = Some(id.to_string());
        self
    }

    pub fn policy(mut self, policy: Policy) -> Self {
        self.0.policy = policy;
        self
    }

    pub fn private_property<T>(mut self, property: &str, value: T) -> Self
    where
        T: ToValue,
    {
        self.0.private_properties.set(property, value);
        self
    }

    pub fn build(self) -> NewPolicyDefinition {
        self.0
    }
}

impl Default for PolicyDefinition {
    fn default() -> Self {
        Self {
            id: String::default(),
            policy: Policy::builder().build(),
            private_properties: Properties::default(),
        }
    }
}

impl Default for NewPolicyDefinition {
    fn default() -> Self {
        Self {
            id: Option::default(),
            policy: Policy::builder().build(),
            private_properties: Properties::default(),
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Policy {
    #[serde(rename = "@id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "@type")]
    kind: PolicyKind,
    #[serde(alias = "odrl:assignee")]
    assignee: Option<String>,
    #[serde(alias = "odrl:assigner")]
    assigner: Option<String>,
    #[serde(alias = "odrl:target")]
    target: Option<Target>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "permission", alias = "odrl:permission", default)]
    permissions: Vec<Permission>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "obligation", alias = "odrl:obligation", default)]
    obligations: Vec<Obligation>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "prohibition", alias = "odrl:prohibition", default)]
    prohibitions: Vec<Prohibition>,
}

impl Policy {
    pub fn builder() -> PolicyBuilder {
        PolicyBuilder(Policy {
            id: None,
            kind: PolicyKind::Set,
            assignee: None,
            assigner: None,
            target: None,
            permissions: vec![],
            obligations: vec![],
            prohibitions: vec![],
        })
    }

    pub fn kind(&self) -> &PolicyKind {
        &self.kind
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn assignee(&self) -> Option<&String> {
        self.assignee.as_ref()
    }

    pub fn assigner(&self) -> Option<&String> {
        self.assigner.as_ref()
    }

    pub fn target(&self) -> Option<&Target> {
        self.target.as_ref()
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn obligations(&self) -> &[Obligation] {
        &self.obligations
    }

    pub fn prohibitions(&self) -> &[Prohibition] {
        &self.prohibitions
    }
}

pub struct PolicyBuilder(Policy);

impl PolicyBuilder {
    pub fn id(mut self, id: &str) -> Self {
        self.0.id = Some(id.to_string());
        self
    }

    pub fn assigner(mut self, assigner: &str) -> Self {
        self.0.assigner = Some(assigner.to_string());
        self
    }

    pub fn target(mut self, target: Target) -> Self {
        self.0.target = Some(target);
        self
    }

    pub fn kind(mut self, kind: PolicyKind) -> Self {
        self.0.kind = kind;
        self
    }

    pub fn permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.0.permissions = permissions;
        self
    }

    pub fn permission(mut self, permission: Permission) -> Self {
        self.0.permissions.push(permission);
        self
    }

    pub fn build(self) -> Policy {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PolicyKind {
    #[serde(alias = "odrl:Set")]
    Set,
    #[serde(alias = "odrl:Offer")]
    Offer,
    #[serde(alias = "odrl:Agreement")]
    Agreement,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Permission {
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "constraint", alias = "odrl:constraint", default)]
    constraints: Vec<Constraint>,
    #[serde(alias = "odrl:action")]
    action: Action,
}

impl Permission {
    pub fn builder() -> PermissionBuilder {
        PermissionBuilder(Permission {
            action: Action::default(),
            constraints: vec![],
        })
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

pub struct PermissionBuilder(Permission);

impl PermissionBuilder {
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.0.constraints = constraints;
        self
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.0.constraints.push(constraint);
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.0.action = action;
        self
    }

    pub fn build(self) -> Permission {
        self.0
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Obligation {
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "constraint", alias = "odrl:constraint", default)]
    constraints: Vec<Constraint>,
    #[serde(alias = "odrl:action")]
    action: Action,
}

impl Obligation {
    pub fn builder() -> ObligationBuilder {
        ObligationBuilder(Obligation {
            action: Action::default(),
            constraints: vec![],
        })
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

pub struct ObligationBuilder(Obligation);

impl ObligationBuilder {
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.0.constraints = constraints;
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.0.action = action;
        self
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.0.constraints.push(constraint);
        self
    }

    pub fn build(self) -> Obligation {
        self.0
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Prohibition {
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(rename = "constraint", alias = "odrl:constraint", default)]
    constraints: Vec<Constraint>,
    #[serde(alias = "odrl:action")]
    action: Action,
}

impl Prohibition {
    pub fn builder() -> ProhibitionBuilder {
        ProhibitionBuilder(Prohibition {
            action: Action::default(),
            constraints: vec![],
        })
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

pub struct ProhibitionBuilder(Prohibition);

impl ProhibitionBuilder {
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.0.constraints = constraints;
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.0.action = action;
        self
    }

    pub fn constraint(mut self, constraint: Constraint) -> Self {
        self.0.constraints.push(constraint);
        self
    }

    pub fn build(self) -> Prohibition {
        self.0
    }
}

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Target {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

impl Target {
    pub fn simple(target: &str) -> Target {
        Target::Simple(target.to_string())
    }

    pub fn id(target: &str) -> Target {
        Target::Id {
            id: target.to_string(),
        }
    }

    pub fn get_id(&self) -> &str {
        match self {
            Target::Simple(target) => target,
            Target::Id { id } => id,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::new("http://www.w3.org/ns/odrl/2/use".to_string())
    }
}

impl Action {
    pub fn id(&self) -> &String {
        match self {
            Action::Simple(id) => id,
            Action::Id { id } => id,
        }
    }
}

impl Action {
    pub fn new(kind: String) -> Self {
        Action::Id { id: kind }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Constraint {
    Atomic(AtomicConstraint),
    MultiplicityConstraint(MultiplicityConstraint),
}

impl Constraint {
    pub fn atomic(atomic: AtomicConstraint) -> Self {
        Constraint::Atomic(atomic)
    }

    pub fn or(constraints: Vec<Constraint>) -> Self {
        Constraint::MultiplicityConstraint(MultiplicityConstraint::Or(constraints))
    }

    pub fn and(constraints: Vec<Constraint>) -> Self {
        Constraint::MultiplicityConstraint(MultiplicityConstraint::And(constraints))
    }

    pub fn xone(constraints: Vec<Constraint>) -> Self {
        Constraint::MultiplicityConstraint(MultiplicityConstraint::Xone(constraints))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum LeftOperand {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

impl LeftOperand {
    pub fn simple(op: &str) -> LeftOperand {
        LeftOperand::Simple(op.to_string())
    }

    pub fn id(op: &str) -> LeftOperand {
        LeftOperand::Id { id: op.to_string() }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AtomicConstraint {
    #[serde(rename = "leftOperand", alias = "odrl:leftOperand")]
    left_operand: LeftOperand,
    #[serde(alias = "odrl:operator")]
    operator: Operator,
    #[serde(rename = "rightOperand", alias = "odrl:rightOperand")]
    right_operand: PropertyValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MultiplicityConstraint {
    Or(Vec<Constraint>),
    And(Vec<Constraint>),
    Xone(Vec<Constraint>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Operator {
    Simple(String),
    Id {
        #[serde(rename = "@id")]
        id: String,
    },
}

impl Operator {
    pub fn simple(op: &str) -> Operator {
        Operator::Simple(op.to_string())
    }

    pub fn id(op: &str) -> Operator {
        Operator::Id { id: op.to_string() }
    }
}

impl AtomicConstraint {
    pub fn new<T: ToValue>(left_operand: &str, operator: &str, right_operand: T) -> Self {
        AtomicConstraint::new_with_operator(
            LeftOperand::Simple(left_operand.to_string()),
            Operator::Simple(operator.to_string()),
            right_operand,
        )
    }

    pub fn new_with_operator<T: ToValue>(
        left_operand: impl Into<LeftOperand>,
        operator: Operator,
        right_operand: T,
    ) -> Self {
        Self {
            left_operand: left_operand.into(),
            operator,
            right_operand: PropertyValue(right_operand.into_value()),
        }
    }
}

impl From<&str> for LeftOperand {
    fn from(value: &str) -> Self {
        LeftOperand::Id {
            id: value.to_string(),
        }
    }
}
