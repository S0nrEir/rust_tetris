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
    
    /// 带颜色的info级别log / info level log with color
    /// # Arguments
    /// * sender - 调用方 / caller
    /// * msg - 消息 / message
    /// * color - 颜色 / color
    pub fn log_info_colored<T:Debug>(sender:T,msg:&str,color: Color) {
        match color {
            Color::White =>   { println!("{} : {}",format!("{:?}",sender).white(),      msg.white()); }
            Color::Yellow =>  { println!("{} : {}",format!("{:?}",sender).yellow(),     msg.bright_yellow()); }
            Color::Red =>     { println!("{} : {}",format!("{:?}",sender).red(),        msg.bright_red()); }
            Color::Magenta => { println!("{} : {}",format!("{:?}",sender).bright_red(), msg.bright_magenta()); }
            Color::Green =>   { println!("{} : {}",format!("{:?}",sender).bright_red(), msg.bright_green()); }
            Color::Cyan  =>   { println!("{} : {}",format!("{:?}",sender).bright_red(), msg.bright_cyan()); }
            _ =>              { println!("{} : {}",format!("{:?}",sender).white(),      msg.white()); }
        }
    }
    
    fn log_info<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).white(), msg.white());
    }
    fn log_warning<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).yellow(), msg.yellow());
    }
    fn log_error<T:Debug>(sender:T,msg:&str){
        println!("{} : {}",format!("{:?}",sender).bright_red(), msg.bright_red());
    }
    fn log_fatal<T:Debug>(sender:T,msg:&str){
        panic!("{} : {}",format!("{:?}",sender).red(), msg.red());
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