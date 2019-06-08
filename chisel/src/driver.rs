use std::collections::HashMap;
use std::path::PathBuf;

use libchisel::ModuleTranslator;
use libchisel::ModuleError;

use parity_wasm::elements::Module;

/// A set of "Runners" to be executed. The instantiated form of a chisel configuration, with an
/// arbitrary number of rulesets.
pub struct ChiselDriver(Vec<Runner>);

/// The instantiated form of a chisel ruleset.
pub enum ChiselRunner {
    /// Normal mode. Accepts a module and runs a set of validators and verifiers on it.
    NormalMode(
        Module,
        ChiselModuleSet,
    ),
    /// This variant is used when a creator module is present, and runs the translator/validator
    /// modules post-creation.
    CreateMode(
        Box<dyn ModuleCreator>,
        ChiselModuleSet,
    )
}

/// A list of Modules to run. Translators are run before the validators by default.
pub struct ChiselModuleSet {
    in_place: bool,
    translators: Vec<Box<dyn ModuleTranslator>>,
    validators: Vec<Box<dyn ModuleValidator>>,
}

impl ChiselModuleSet {
    /// Runs all the translators in the module set. On success, returns a Vec of results describing
    /// the status of the contained chisel modules' executions.
    pub fn run_translators(&mut self, module: &mut Module) -> Vec<Result<bool, ModuleError>> {
        let mut ret: Vec<Result<bool, ModuleError>> = Vec::new();
        for chisel_mod in self.translators.iter() {
            if self.in_place {
                match chisel_mod.translate_inplace(&mut module) {
                    Ok(mutated) => ret.push(Ok(mutated)),
                    Err(e) => if e == ModuleError::NotSupported {
                        // If we allow to fallback to functional mode, try doing so 
                    }
                }
            } else {
                // translate functionally
            }
        }
    }
}
