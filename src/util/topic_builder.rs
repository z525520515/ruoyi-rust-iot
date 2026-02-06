use urlencoding::encode;

#[allow(dead_code)]
pub struct TopicBuilder {
    device_hash: String,
    client_name: String,
}

impl TopicBuilder {
    /// 创建主题构建器实例
    pub fn new(device_hash: &str, client_name: &str) -> Self {
        Self {
            device_hash: encode(device_hash).to_string(),
            client_name: encode(client_name).to_string(),
        }
    }

    /// 获取主题前缀: {deviceUniqueIDHash}/{clientName}
    pub fn get_prefix(&self) -> String {
        format!("{}/{}", self.device_hash, self.client_name)
    }

    /// 遗嘱主题: {topicPrefix}/will
    pub fn will(&self) -> String {
        format!("{}/will", self.get_prefix())
    }

    /// 控制协议栈主题基路径
    fn control_stack_base(&self, protocol_type: &str, stack_name: &str) -> String {
        let encoded_type = encode(protocol_type);
        let encoded_name = encode(stack_name);
        format!(
            "{}/protocol_stack/{}/{}",
            self.get_prefix(),
            encoded_type,
            encoded_name
        )
    }

    /// 控制请求主题: {topicPrefix}/protocol_stack/{type}/{name}/{action}/request
    pub fn control_request(
        &self,
        protocol_type: &str,
        stack_name: &str,
        action: &str,
    ) -> String {
        format!(
            "{}/{}/request",
            self.control_stack_base(protocol_type, stack_name),
            encode(action)
        )
    }

    /// 控制响应主题: {topicPrefix}/protocol_stack/{type}/{name}/{action}/response
    pub fn control_response(
        &self,
        protocol_type: &str,
        stack_name: &str,
        action: &str,
    ) -> String {
        format!(
            "{}/{}/response",
            self.control_stack_base(protocol_type, stack_name),
            encode(action)
        )
    }

    /// 日志消息主题: {topicPrefix}/protocol_stack/{type}/{name}/logger/new_log_message
    pub fn logger_new_message(
        &self,
        protocol_type: &str,
        stack_name: &str,
    ) -> String {
        format!(
            "{}/logger/new_log_message",
            self.control_stack_base(protocol_type, stack_name)
        )
    }

    /// 维护工具 - 检查更新完成: {topicPrefix}/protocol_stack/{type}/{name}/maintenance_tool/check_updates/finished
    pub fn maintenance_check_updates_finished(
        &self,
        protocol_type: &str,
        stack_name: &str,
    ) -> String {
        format!(
            "{}/maintenance_tool/check_updates/finished",
            self.control_stack_base(protocol_type, stack_name)
        )
    }

    /// 维护工具 - 更新完成: {topicPrefix}/protocol_stack/{type}/{name}/maintenance_tool/update/finished
    pub fn maintenance_update_finished(
        &self,
        protocol_type: &str,
        stack_name: &str,
    ) -> String {
        format!(
            "{}/maintenance_tool/update/finished",
            self.control_stack_base(protocol_type, stack_name)
        )
    }

    /// 维护工具 - 更新标准输出: {topicPrefix}/protocol_stack/{type}/{name}/maintenance_tool/update/standard_output
    pub fn maintenance_update_stdout(
        &self,
        protocol_type: &str,
        stack_name: &str,
    ) -> String {
        format!(
            "{}/maintenance_tool/update/standard_output",
            self.control_stack_base(protocol_type, stack_name)
        )
    }

    /// 获取所有需要订阅的通知主题
    pub fn get_all_notification_topics(
        &self,
        protocol_type: &str,
        stack_name: &str,
    ) -> Vec<(String, rumqttc::QoS)> {
        vec![
            (self.logger_new_message(protocol_type, stack_name), rumqttc::QoS::AtLeastOnce),
            (
                self.maintenance_check_updates_finished(protocol_type, stack_name),
                rumqttc::QoS::AtLeastOnce,
            ),
            (
                self.maintenance_update_finished(protocol_type, stack_name),
                rumqttc::QoS::AtLeastOnce,
            ),
            (
                self.maintenance_update_stdout(protocol_type, stack_name),
                rumqttc::QoS::AtLeastOnce,
            ),
        ]
    }
}
