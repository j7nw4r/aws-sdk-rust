// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AttachInstancesToLoadBalancer`](crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`load_balancer_name(impl Into<String>)`](crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder::load_balancer_name) / [`set_load_balancer_name(Option<String>)`](crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder::set_load_balancer_name):<br>required: **true**<br><p>The name of the load balancer.</p><br>
    ///   - [`instance_names(impl Into<String>)`](crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder::instance_names) / [`set_instance_names(Option<Vec::<String>>)`](crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder::set_instance_names):<br>required: **true**<br><p>An array of strings representing the instance name(s) you want to attach to your load balancer.</p>  <p>An instance must be <code>running</code> before you can attach it to your load balancer.</p>  <p>There are no additional limits on the number of instances you can attach to your load balancer, aside from the limit of Lightsail instances you can create in your account (20).</p><br>
    /// - On success, responds with [`AttachInstancesToLoadBalancerOutput`](crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerOutput) with field(s):
    ///   - [`operations(Option<Vec::<Operation>>)`](crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<AttachInstancesToLoadBalancerError>`](crate::operation::attach_instances_to_load_balancer::AttachInstancesToLoadBalancerError)
    pub fn attach_instances_to_load_balancer(
        &self,
    ) -> crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder {
        crate::operation::attach_instances_to_load_balancer::builders::AttachInstancesToLoadBalancerFluentBuilder::new(self.handle.clone())
    }
}
