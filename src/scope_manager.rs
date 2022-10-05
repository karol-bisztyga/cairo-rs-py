use std::{any::Any, collections::HashMap};

use cairo_rs::{
    any_box, hint_processor::proxies::exec_scopes_proxy::ExecutionScopesProxy,
    vm::errors::vm_errors::VirtualMachineError,
};
use pyo3::{pyclass, pymethods, PyObject};

#[pyclass(unsendable)]
#[derive(Debug, Clone)]
pub struct PyEnterScope {
    new_scopes: Vec<HashMap<String, PyObject>>,
}

impl PyEnterScope {
    pub fn new() -> PyEnterScope {
        PyEnterScope {
            new_scopes: Vec::new(),
        }
    }

    pub fn update_scopes(
        &self,
        scopes: &mut ExecutionScopesProxy,
    ) -> Result<(), VirtualMachineError> {
        for scope_variables in self.new_scopes.iter() {
            let mut new_scope = HashMap::<String, Box<dyn Any>>::new();
            for (name, pyobj) in scope_variables {
                new_scope.insert(name.to_string(), any_box!(pyobj.clone()));
            }
            scopes.enter_scope(new_scope);
        }
        Ok(())
    }
}

#[pymethods]
impl PyEnterScope {
    pub fn __call__(&mut self, variables: Option<HashMap<String, PyObject>>) {
        match variables {
            Some(variables) => self.new_scopes.push(variables),
            None => self.new_scopes.push(HashMap::new()),
        }
    }
}

#[pyclass(unsendable)]
#[derive(Debug, Clone)]
pub struct PyExitScope {
    num: i32,
}

impl PyExitScope {
    pub fn new() -> PyExitScope {
        PyExitScope { num: 0 }
    }

    pub fn update_scopes(
        &self,
        scopes: &mut ExecutionScopesProxy,
    ) -> Result<(), VirtualMachineError> {
        for _ in 0..self.num {
            scopes.exit_scope()?
        }
        Ok(())
    }
}

#[pymethods]
impl PyExitScope {
    pub fn __call__(&mut self) {
        self.num += 1
    }
}