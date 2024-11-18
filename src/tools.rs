pub mod Logger{
    use std::fmt::Debug;
    ///打印工具
    /// # Arguments
    /// * sender - 调用方 / caller
    /// * msg - 消息 / message
    /// # Returns
    /// none
    pub fn log<T:Debug>(sender:T,msg:&str) {
        println!("{:?} {}",sender,msg);
    }
}