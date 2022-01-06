#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "actionType")]
    pub action_type: action::ActionType,
}
pub mod action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        AddActionGroups,
        RemoveAllActionGroups,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddActionGroups {
    #[serde(flatten)]
    pub action: Action,
    #[serde(rename = "actionGroupIds")]
    pub action_group_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProcessingRule {
    #[serde(flatten)]
    pub managed_resource: ManagedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProcessingRuleProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProcessingRuleProperties {
    pub scopes: Scopes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Conditions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    pub actions: Vec<Action>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProcessingRulesList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AlertProcessingRule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<condition::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<condition::Operator>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
pub mod condition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Field {
        Severity,
        MonitorService,
        MonitorCondition,
        SignalType,
        TargetResourceType,
        TargetResource,
        TargetResourceGroup,
        AlertRuleId,
        AlertRuleName,
        Description,
        AlertContext,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        Equals,
        NotEquals,
        Contains,
        DoesNotContain,
    }
}
pub type Conditions = Vec<Condition>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DailyRecurrence {
    #[serde(flatten)]
    pub recurrence: Recurrence,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DaysOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorServiceDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorServiceList {
    #[serde(flatten)]
    pub alerts_meta_data_properties: AlertsMetaDataProperties,
    pub data: Vec<MonitorServiceDetails>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyRecurrence {
    #[serde(flatten)]
    pub recurrence: Recurrence,
    #[serde(rename = "daysOfMonth")]
    pub days_of_month: Vec<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    #[serde(rename = "recurrenceType")]
    pub recurrence_type: recurrence::RecurrenceType,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}
pub mod recurrence {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RecurrenceType {
        Daily,
        Weekly,
        Monthly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveAllActionGroups {
    #[serde(flatten)]
    pub action: Action,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    #[serde(rename = "effectiveFrom", default, skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<String>,
    #[serde(rename = "effectiveUntil", default, skip_serializing_if = "Option::is_none")]
    pub effective_until: Option<String>,
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrences: Vec<Recurrence>,
}
pub type Scopes = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeeklyRecurrence {
    #[serde(flatten)]
    pub recurrence: Recurrence,
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Vec<DaysOfWeek>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertContext {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertModificationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertModificationItem {
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<alert_modification_item::ModificationEvent>,
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod alert_modification_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        AlertCreated,
        StateChange,
        MonitorConditionChange,
        SeverityChange,
        ActionRuleTriggered,
        ActionRuleSuppressed,
        ActionsTriggered,
        ActionsSuppressed,
        ActionsFailed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertModificationProperties {
    #[serde(rename = "alertId", default, skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifications: Vec<AlertModificationItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub essentials: Option<Essentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<AlertContext>,
    #[serde(rename = "egressConfig", default, skip_serializing_if = "Option::is_none")]
    pub egress_config: Option<EgressConfig>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsMetaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsMetaDataProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsMetaDataProperties {
    #[serde(rename = "metadataIdentifier")]
    pub metadata_identifier: alerts_meta_data_properties::MetadataIdentifier,
}
pub mod alerts_meta_data_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MetadataIdentifier {
        MonitorServiceList,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsSummaryGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "smartGroupsCount", default, skip_serializing_if = "Option::is_none")]
    pub smart_groups_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupedby: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<AlertsSummaryGroupItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSummaryGroupItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groupedby: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<AlertsSummaryGroupItem>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EgressConfig {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Essentials {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<essentials::Severity>,
    #[serde(rename = "signalType", default, skip_serializing_if = "Option::is_none")]
    pub signal_type: Option<essentials::SignalType>,
    #[serde(rename = "alertState", default, skip_serializing_if = "Option::is_none")]
    pub alert_state: Option<essentials::AlertState>,
    #[serde(rename = "monitorCondition", default, skip_serializing_if = "Option::is_none")]
    pub monitor_condition: Option<essentials::MonitorCondition>,
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<String>,
    #[serde(rename = "targetResourceName", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_name: Option<String>,
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(rename = "targetResourceType", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_type: Option<String>,
    #[serde(rename = "monitorService", default, skip_serializing_if = "Option::is_none")]
    pub monitor_service: Option<essentials::MonitorService>,
    #[serde(rename = "alertRule", default, skip_serializing_if = "Option::is_none")]
    pub alert_rule: Option<String>,
    #[serde(rename = "sourceCreatedId", default, skip_serializing_if = "Option::is_none")]
    pub source_created_id: Option<String>,
    #[serde(rename = "smartGroupId", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_id: Option<String>,
    #[serde(rename = "smartGroupingReason", default, skip_serializing_if = "Option::is_none")]
    pub smart_grouping_reason: Option<String>,
    #[serde(rename = "startDateTime", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "monitorConditionResolvedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub monitor_condition_resolved_date_time: Option<String>,
    #[serde(rename = "lastModifiedUserName", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user_name: Option<String>,
}
pub mod essentials {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SignalType {
        Metric,
        Log,
        Unknown,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AlertState {
        New,
        Acknowledged,
        Closed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorCondition {
        Fired,
        Resolved,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MonitorService {
        #[serde(rename = "Application Insights")]
        ApplicationInsights,
        #[serde(rename = "ActivityLog Administrative")]
        ActivityLogAdministrative,
        #[serde(rename = "ActivityLog Security")]
        ActivityLogSecurity,
        #[serde(rename = "ActivityLog Recommendation")]
        ActivityLogRecommendation,
        #[serde(rename = "ActivityLog Policy")]
        ActivityLogPolicy,
        #[serde(rename = "ActivityLog Autoscale")]
        ActivityLogAutoscale,
        #[serde(rename = "Log Analytics")]
        LogAnalytics,
        Nagios,
        Platform,
        #[serde(rename = "SCOM")]
        Scom,
        ServiceHealth,
        SmartDetector,
        #[serde(rename = "VM Insights")]
        VmInsights,
        Zabbix,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmartGroupProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupAggregatedProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupModification {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmartGroupModificationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupModificationItem {
    #[serde(rename = "modificationEvent", default, skip_serializing_if = "Option::is_none")]
    pub modification_event: Option<smart_group_modification_item::ModificationEvent>,
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[serde(rename = "modifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "modifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod smart_group_modification_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ModificationEvent {
        SmartGroupCreated,
        StateChange,
        AlertAdded,
        AlertRemoved,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupModificationProperties {
    #[serde(rename = "smartGroupId", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifications: Vec<SmartGroupModificationItem>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupProperties {
    #[serde(rename = "alertsCount", default, skip_serializing_if = "Option::is_none")]
    pub alerts_count: Option<i64>,
    #[serde(rename = "smartGroupState", default, skip_serializing_if = "Option::is_none")]
    pub smart_group_state: Option<smart_group_properties::SmartGroupState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<smart_group_properties::Severity>,
    #[serde(rename = "startDateTime", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "lastModifiedUserName", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "monitorServices", default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_services: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "monitorConditions", default, skip_serializing_if = "Vec::is_empty")]
    pub monitor_conditions: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "alertStates", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_states: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "alertSeverities", default, skip_serializing_if = "Vec::is_empty")]
    pub alert_severities: Vec<SmartGroupAggregatedProperty>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
pub mod smart_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SmartGroupState {
        New,
        Acknowledged,
        Closed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Sev0,
        Sev1,
        Sev2,
        Sev3,
        Sev4,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartGroupsList {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SmartGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}