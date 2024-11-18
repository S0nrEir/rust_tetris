pub mod Logger{
    use std::fmt::Debug;
    use colored::*;
    use num_enum::IntoPrimitive;
    ///打印工具
    /// # Arguments
    /// * sender - 调用方 / caller
    /// * msg - 消息 / message
    /// * level - 日志级别 / log level
    /// # Returns
    /// none
    pub fn log<T:Debug>(sender:T,msg:&str,level:LogLevelEnum) {
        match level {
            LogLevelEnum::Info => { log_info(sender,msg); }
            LogLevelEnum::Warning => { log_warning(sender,msg); }
            LogLevelEnum::Error => { log_error(sender,msg); }
            LogLevelEnum::Fatal => { log_fatal(sender,msg); }
            _ => {}
        }
    }
    
    fn log_info<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).white(), msg.white());
    }
    fn log_warning<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).yellow(), msg.yellow());
    }
    fn log_error<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).red(), msg.red());
    }
    fn log_fatal<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).bright_red(), msg.bright_red());
    }    
    
    #[derive(IntoPrimitive)]
    #[repr(i32)]
    ///日志级别 / log level
    pub enum LogLevelEnum{
        ///信息 / info
        Info,
        ///警告 / warning
        Warning,
        ///错误 / error
        Error,
        ///致命 / fatal
        Fatal
    }
}