pub mod procedure_main_ui;
pub mod procedure_playing;
pub mod procedure_over;
pub mod t_procedure_param;

use std::collections::HashMap;
use std::mem;
use ggez::{Context, GameResult};
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::procedure::t_procedure_param::ProcedureParam;
use crate::t_state::TState;
use crate::tools::logger::*;
use crate::t_updatable::{Tickable, Updatable};

/// 流程组件，用于控制流程 / procedure component, used to control procedure
#[derive(Debug)]
pub struct ProcedureComponent{
    ///当前流程 / current procedure
    _current_procedure:Option<Box<dyn TState>>,
    // _current_procedure_index:i32,
    ///流程映射 / procedure map
    _procedure_map:HashMap<i32,Box<dyn TState>>,
}

impl ProcedureComponent {
    /// 创建一个流程组件，添加流程失败将会终止并直接返回 / create a procedure component and add procedure,
    /// failed will terminate and return directly
    /// #Arguments
    /// * `entry_procedure_index` - 初始流程枚举 / initial procedure enum
    /// * `procedure_vec` - 要包含的流程列表 / procedure list to include
    pub fn new(procedure_vec:Vec<Option<Box<dyn TState>>>) -> Self{
        
        let procedure_map = HashMap::new();
        let mut procedure_component = ProcedureComponent{
            _current_procedure:None,
            // _current_procedure_index:entry_procedure_index,
            _procedure_map:procedure_map,
        };
        
        for item in procedure_vec {
            match procedure_component.add_exist_procedure(item){
                Ok(_) => {},
                Err(err_msg) => { 
                    log(&procedure_component,&err_msg,LogLevelEnum::Error);
                    break;
                },
            };
        }
        // procedure_component.set_default_procedure(entry_procedure_index);
        #[cfg(feature = "debug_log")]
        {
            let all_procedure = procedure_component.all_procedure_name();
            log(&procedure_component,&format!("all procedure : {:?}",all_procedure),LogLevelEnum::Info);
        }
        
        return procedure_component;
    }

    ///创建一个不带任何流程的流程组件 / create a procedure component without any procedure
    pub fn new_as_empty() -> Self {
        let procedure_component = ProcedureComponent{
            _current_procedure:None,
            // _current_procedure_index : -1,
            _procedure_map : HashMap::new(),
        };
        return procedure_component;
    }
    
    /// 添加一个已存在的流程 / add an existing procedure
    fn add_exist_procedure(&mut self,procedure:Option<Box<dyn TState>>) -> Result<(),String>{
        if(procedure.is_none()){
            return Err(String::from("procedure is none"));
        }
        
        let enum_type = procedure.as_ref().unwrap().get_state().into();
        if(self._procedure_map.contains_key(&enum_type)){
            let err_msg = String::from(format!("procedure already exists, procedure:{}",enum_type));
            return Err(err_msg);
        }

        self._procedure_map.insert(enum_type,procedure.unwrap());
        return Ok(());
    }
    
    /// 添加一个流程 / add a procedure
    /// #Arguments
    /// * `procedure_enum` - 要添加的流程枚举 / procedure enum to add
    /// #Return
    /// * `bool` - 是否添加成功 / whether add successfully
    pub fn add_new_procedure(&mut self,procedure_enum:ProcedureEnum) -> bool{
        let enum_type:i32 = procedure_enum.into();
        if(self._procedure_map.contains_key(&enum_type)){
            log(self,"procedure already exists",LogLevelEnum::Info);
            return false;
        }
        let mut insert_succ : Option<Box<dyn TState>> = None;
        match procedure_enum {
            ProcedureEnum::MainUI => {
                insert_succ = self._procedure_map.insert(enum_type,Box::new(procedure_main_ui::ProcedureMainUI::new()));
            },
            ProcedureEnum::Playing => {
                insert_succ = self._procedure_map.insert(enum_type,Box::new(procedure_playing::ProcedurePlaying::new()));
            },
            ProcedureEnum::Over => {
                insert_succ = self._procedure_map.insert(enum_type,Box::new(procedure_over::ProcedureOver::new()));
            },
        }
        return insert_succ.is_some();
    }
    
    /// 切换流程 / switch procedure
    /// #Arguments
    /// * `procedure_to_switch` - 要切换的流程枚举 / procedure enum to switch
    /// #Return
    /// * `bool` - 是否切换成功 / whether switch successfully
    pub fn switch(&mut self,
                  procedure_to_switch:ProcedureEnum,
                  enter_param:Box<dyn ProcedureParam>,
                  leave_param:Option<Box<dyn ProcedureParam>>) -> bool{

        //离开原有流程，并且将其保存起来 / leave the original procedure and save it
        //ps 对于第一次理解所有权概念的人来说真是折磨!! / ps for those who first understand the concept of ownership, it is really torture!!
        if let Some(procedure_to_leave) = mem::replace(&mut self._current_procedure,None){
            let procedure_enum = procedure_to_leave.get_state();
            procedure_to_leave.on_leave(leave_param);
            //如果k已经存在则更新并返回旧值，如果是全新的则返回none
            match self._procedure_map.insert(procedure_enum.into(), procedure_to_leave) { 
                Some(_) => {
                    log("procedure.rs",&format!("switch() ---> contains old value while leave,procedure enum:{:?}",procedure_enum),LogLevelEnum::Error);
                    return false;
                },
                _ => {}
            }
        }
        
        //获取新流程并进入 / get new procedure and enter
        let enum_type:i32 = procedure_to_switch.into();
        //remove返回被移除的值并且获取所有权，如果不存在返回none
        return if let Some(new_procedure) = self._procedure_map.remove(&enum_type) {
            self._current_procedure = Some(new_procedure);
            let procedure_ref = self._current_procedure.as_mut().unwrap();
            procedure_ref.on_enter(enter_param);
            // self._current_procedure.as_ref().unwrap().on_enter(Some(Box::new(ProcedureMainUI::new())));
            true
        }
        else {
            let err_msg = &format!("procedure not found , procedure:{}", enum_type);
            log(self, err_msg, LogLevelEnum::Error);
            false
        }
    }
    
    /// 当前的流程阶段 / current procedure stage
    /// #Return
    /// * `Option<ProcedureEnuxm>` - 当前流程 / current procedure
    pub fn curr_procedure(&self) -> Option<ProcedureEnum>{
        if(self._current_procedure.is_none()){
            return None;
        }
        return Some(self._current_procedure.as_ref().unwrap().get_state().into());
    }
    
    /// 获取所有已添加的流程 / get all added procedures
    /// #Return
    /// * `Vec<String>` - 所有流程名称 / all procedure names
    pub fn all_procedure_name(&self) -> Vec<String>{
        let mut procedure_name_list : Vec<String> = Vec::new();
        let values = self._procedure_map.values();
        values.for_each(|value|{
            procedure_name_list.push(String::from(value.get_state().as_str()));
        });
        // for (key,value) in self._procedure_map.iter(){
        //     procedure_name_list.push(String::from(value.get_state().as_str()));
        // }
        return procedure_name_list;
    }
    
    /// 绘制接口 / draw interface
    /// #Arguments
    /// * `ctx` - 上下文对象 / context object
    /// #Return
    /// * `GameResult` - 处理结果 / processing result
    pub fn on_draw(&mut self,ctx:&mut Context) -> GameResult{
        if let Some(curr_procedure) = &mut self._current_procedure{
            curr_procedure.on_draw(ctx);
        }
        
        return Ok(());
    }
}

impl Tickable for ProcedureComponent {
    fn on_tick(&mut self,ctx:&mut Context,delta_time:f32,interval:f32) {
        #[cfg(feature = "debug_log")]{
            crate::tools::logger::log_info_colored("ProcedureComponent.on_tick()", &format!("calling..."), Color::Cyan);
        }
    }
}
