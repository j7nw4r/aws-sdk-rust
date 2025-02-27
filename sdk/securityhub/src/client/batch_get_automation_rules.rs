// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetAutomationRules`](crate::operation::batch_get_automation_rules::builders::BatchGetAutomationRulesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`automation_rules_arns(impl Into<String>)`](crate::operation::batch_get_automation_rules::builders::BatchGetAutomationRulesFluentBuilder::automation_rules_arns) / [`set_automation_rules_arns(Option<Vec::<String>>)`](crate::operation::batch_get_automation_rules::builders::BatchGetAutomationRulesFluentBuilder::set_automation_rules_arns):<br>required: **true**<br><p> A list of rule ARNs to get details for. </p><br>
    /// - On success, responds with [`BatchGetAutomationRulesOutput`](crate::operation::batch_get_automation_rules::BatchGetAutomationRulesOutput) with field(s):
    ///   - [`rules(Option<Vec::<AutomationRulesConfig>>)`](crate::operation::batch_get_automation_rules::BatchGetAutomationRulesOutput::rules): <p> A list of rule details for the provided rule ARNs. </p>
    ///   - [`unprocessed_automation_rules(Option<Vec::<UnprocessedAutomationRule>>)`](crate::operation::batch_get_automation_rules::BatchGetAutomationRulesOutput::unprocessed_automation_rules): <p> A list of objects containing <code>RuleArn</code>, <code>ErrorCode</code>, and <code>ErrorMessage</code>. This parameter tells you which automation rules the request didn't retrieve and why. </p>
    /// - On failure, responds with [`SdkError<BatchGetAutomationRulesError>`](crate::operation::batch_get_automation_rules::BatchGetAutomationRulesError)
    pub fn batch_get_automation_rules(&self) -> crate::operation::batch_get_automation_rules::builders::BatchGetAutomationRulesFluentBuilder {
        crate::operation::batch_get_automation_rules::builders::BatchGetAutomationRulesFluentBuilder::new(self.handle.clone())
    }
}
