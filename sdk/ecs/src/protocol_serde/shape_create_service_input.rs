// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_service_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_service::CreateServiceInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.service_name {
        object.key("serviceName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.task_definition {
        object.key("taskDefinition").string(var_3.as_str());
    }
    if let Some(var_4) = &input.load_balancers {
        let mut array_5 = object.key("loadBalancers").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_load_balancer::ser_load_balancer(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.service_registries {
        let mut array_9 = object.key("serviceRegistries").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_service_registry::ser_service_registry(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.desired_count {
        object.key("desiredCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.client_token {
        object.key("clientToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.launch_type {
        object.key("launchType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.capacity_provider_strategy {
        let mut array_16 = object.key("capacityProviderStrategy").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_capacity_provider_strategy_item::ser_capacity_provider_strategy_item(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.platform_version {
        object.key("platformVersion").string(var_19.as_str());
    }
    if let Some(var_20) = &input.role {
        object.key("role").string(var_20.as_str());
    }
    if let Some(var_21) = &input.deployment_configuration {
        #[allow(unused_mut)]
        let mut object_22 = object.key("deploymentConfiguration").start_object();
        crate::protocol_serde::shape_deployment_configuration::ser_deployment_configuration(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.placement_constraints {
        let mut array_24 = object.key("placementConstraints").start_array();
        for item_25 in var_23 {
            {
                #[allow(unused_mut)]
                let mut object_26 = array_24.value().start_object();
                crate::protocol_serde::shape_placement_constraint::ser_placement_constraint(&mut object_26, item_25)?;
                object_26.finish();
            }
        }
        array_24.finish();
    }
    if let Some(var_27) = &input.placement_strategy {
        let mut array_28 = object.key("placementStrategy").start_array();
        for item_29 in var_27 {
            {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::protocol_serde::shape_placement_strategy::ser_placement_strategy(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    if let Some(var_31) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_32 = object.key("networkConfiguration").start_object();
        crate::protocol_serde::shape_network_configuration::ser_network_configuration(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.health_check_grace_period_seconds {
        object.key("healthCheckGracePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    if let Some(var_34) = &input.scheduling_strategy {
        object.key("schedulingStrategy").string(var_34.as_str());
    }
    if let Some(var_35) = &input.deployment_controller {
        #[allow(unused_mut)]
        let mut object_36 = object.key("deploymentController").start_object();
        crate::protocol_serde::shape_deployment_controller::ser_deployment_controller(&mut object_36, var_35)?;
        object_36.finish();
    }
    if let Some(var_37) = &input.tags {
        let mut array_38 = object.key("tags").start_array();
        for item_39 in var_37 {
            {
                #[allow(unused_mut)]
                let mut object_40 = array_38.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_40, item_39)?;
                object_40.finish();
            }
        }
        array_38.finish();
    }
    if let Some(var_41) = &input.enable_ecs_managed_tags {
        object.key("enableECSManagedTags").boolean(*var_41);
    }
    if let Some(var_42) = &input.propagate_tags {
        object.key("propagateTags").string(var_42.as_str());
    }
    if let Some(var_43) = &input.enable_execute_command {
        object.key("enableExecuteCommand").boolean(*var_43);
    }
    if let Some(var_44) = &input.service_connect_configuration {
        #[allow(unused_mut)]
        let mut object_45 = object.key("serviceConnectConfiguration").start_object();
        crate::protocol_serde::shape_service_connect_configuration::ser_service_connect_configuration(&mut object_45, var_44)?;
        object_45.finish();
    }
    Ok(())
}
