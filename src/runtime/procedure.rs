﻿pub mod procedure_main_ui;
pub mod procedure_playing;
pub mod procedure_over;

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::define::enum_define::ProcedureEnum;
use crate::runtime::app_components::AppComponents;
use crate::t_state::TState;
use crate::tools::Logger::*;
use crate::t_updatable::Updatable;
use crate::runtime::controller;

/// 流程组件，用于控制流程 / procedure component, used to control procedure
#[derive(Debug)]
pub struct ProcedureComponent{
    ///当前流程 / current procedure
    _current_procedure:Option<Rc<dyn TState>>,
    ///流程映射 / procedure map
    _procedure_map:HashMap<i32,Rc<dyn TState>>,
    ///关联的运行时组件集合 / Associated runtime component set
    _runtime_app_components: Option<Rc<RefCell<AppComponents>>>
}

impl ProcedureComponent {
    /// 创建一个流程组件，添加流程失败将会终止并直接返回 / create a procedure component and add procedure,
    /// failed will terminate and return directly
    /// #Arguments
    /// * `entry_procedure_index` - 初始流程枚举 / initial procedure enum
    /// * `procedure_vec` - 要包含的流程列表 / procedure list to include
    pub fn new(entry_procedure_index:i32,procedure_vec:Vec<Option<Rc<dyn TState>>>) -> Self{
        
        let procedure_map = HashMap::new();
        let mut procedure_component = ProcedureComponent{
            _current_procedure:None,
            _procedure_map:procedure_map,
            _runtime_app_components:None
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
        procedure_component.set_default_procedure(entry_procedure_index);
        
        #[cfg(feature = "debug_log")]
        {
            let all_procedure = procedure_component.all_procedure_name();
            log(&procedure_component,&format!("all procedure : {:?}",all_procedure),LogLevelEnum::Info);
        }
        
        return procedure_component;
    }

    ///创建一个不带任何流程的流程组件 / create a procedure component without any procedure
    pub fn new_as_empty() -> Self {
        let mut procedure_component = ProcedureComponent{
            _current_procedure:None,
            _procedure_map:HashMap::new(),
            _runtime_app_components:None
        };
        return procedure_component;
    }
    
    /// 添加一个已存在的流程 / add an existing procedure
    fn add_exist_procedure(&mut self,procedure:Option<Rc<dyn TState>>) -> Result<(),String>{
        if(procedure.is_none()){
            return Err(String::from("procedure is none"));
        }
        
        let enum_type = procedure.as_ref().unwrap().get_state().into();
        if(self._procedure_map.contains_key(&enum_type)){
            let err_msg = String::from(format!("procedure already exists, procedure:{}",enum_type));
            return Err(err_msg);
        }
        
        let insert_succ = self._procedure_map.insert(enum_type,Rc::clone(procedure.as_ref().unwrap()));
        return Ok(());
        // match insert_succ {
        //     Some(_) => {
        //         return Ok(());
        //     },
        //     None => {
        //         return Err(String::from("insert failed,option is none"));
        //     },
        // }
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
        let mut insert_succ : Option<Rc<dyn TState>> = None;
        match procedure_enum {
            ProcedureEnum::MainUI => {
                insert_succ = self._procedure_map.insert(enum_type,Rc::new(procedure_main_ui::ProcedureMainUI::new()));
            },
            ProcedureEnum::Playing => {
                insert_succ = self._procedure_map.insert(enum_type,Rc::new(procedure_playing::ProcedurePlaying::new()));
            },
            ProcedureEnum::Over => {
                insert_succ = self._procedure_map.insert(enum_type,Rc::new(procedure_over::ProcedureOver::new()));
            },
        }
        return insert_succ.is_some();
    }
    
    /// 切换流程 / switch procedure
    /// #Arguments
    /// * `procedure_to_switch` - 要切换的流程枚举 / procedure enum to switch
    /// #Return
    /// * `bool` - 是否切换成功 / whether switch successfully
    pub fn switch(&mut self,procedure_to_switch:ProcedureEnum) -> bool{
        if let Some(procedure_to_leave) = &self._current_procedure {
            procedure_to_leave.on_leave();
        }
        
        let enum_type:i32 = procedure_to_switch.into();
        if let Some(new_procedure) = self._procedure_map.get(&enum_type){
            self._current_procedure = Some(Rc::clone(new_procedure));
            self._current_procedure.as_ref().unwrap().on_enter();
        }
        else{
            let err_msg = &format!("procedure not found , procedure:{}",enum_type);
            log(self,err_msg,LogLevelEnum::Error);
            return false;
        }
        
        return true;
    }
    
    ///设置默认的procedure
    fn set_default_procedure(&mut self,procedure_type:i32){
        if(!self._procedure_map.contains_key(&procedure_type)){
            log(self,&format!("set default procedure faild,proc type : {}",procedure_type),LogLevelEnum::Error);
            return;
        }
        let temp = Rc::clone(self._procedure_map.get(&procedure_type).unwrap());
        self._current_procedure = Some(Rc::clone(self._procedure_map.get(&procedure_type).unwrap()));
        self._current_procedure.as_ref().unwrap().on_enter();
    }
    
    /// 获取所有当前流程 / get all current procedures
    /// #Return
    /// * `Vec<String>` - 流程名称列表 / procedure name list
    //如果一个方法返回的引用没有指向任何参数，那么它的返回值只能是方法体内部创建的
    //但这会导致一个问题：返回值将在方法结束时离开作用域并被Rust清理，这是一个悬垂引用
    // pub fn all_procedure_name<'a>(&'a self) -> Vec<Cow<'a, str>>{
    //     let mut procedure_name_list= Vec::new();
    //     for (key,value) in self._procedure_map.iter(){
    //         procedure_name_list.push(Cow::Borrowed(value.get_state().as_str()));
    //     }
    //     return procedure_name_list;
    // }

    pub fn all_procedure_name(&self) -> Vec<String>{
        let mut procedure_name_list : Vec<String> = Vec::new();
        for (key,value) in self._procedure_map.iter(){
            procedure_name_list.push(String::from(value.get_state().as_str()));
        }
        return procedure_name_list;
    }
}

impl Updatable for ProcedureComponent{
    fn on_update(&self) {
        todo!()
    }
}

impl ProcedureComponent {
    ///关联运行时组件集合 / Associated runtime component set
    /// #Arguments
    /// - app_components: 运行时组件集合 / Runtime component set
    /// #Returns
    /// - 是否成功 / Is it successful
    pub fn set_components(&mut self,app_components:Rc<RefCell<AppComponents>>) -> bool{
        self._runtime_app_components = Some(app_components);
        return true;
    }
}