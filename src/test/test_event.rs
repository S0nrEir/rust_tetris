// #[cfg(test)]
// mod test_event{
//     use crate::event::EventComponent;
//     use crate::tools::logger::{log, LogLevelEnum};
    
//     #[test]
//     fn entry(){
//         let mut event = EventComponent::new();
//         event.register_event(1, test_event);
//         event.fire(1);
//         event.unregister_event(1,test_event);
//         event.fire(1);
//     }
    
//     #[test]
//     fn test_event(){
//         log("", "event 1 fired", LogLevelEnum::Info);
//     }
// }