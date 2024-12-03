///测试logger相关 / Test logger related
#[cfg(test)]
mod test_logger{
    use crate::tools::logger::*;
    #[test]
    fn entry(){
        log(1u8,"test log warning",LogLevelEnum::Warning);
        log(0u8,"test log info",LogLevelEnum::Info);
        log(2u8,"test log error",LogLevelEnum::Error);
        log(3u8,"test log fatal",LogLevelEnum::Fatal);
    }
}