use std::error::Error;

use libchisel::ModuleCreator;
use libchisel::ModuleError;
use libchisel::ModuleTranslator;
use libchisel::ModuleValidator;

use parity_wasm::elements::Module;

/// A set of "Runners" to be executed. The instantiated form of a chisel configuration, with an
/// arbitrary number of rulesets.
pub struct ChiselDriver<'a>(Vec<ChiselRunner<'a>>);

/// The instantiated form of a chisel ruleset.
pub enum ChiselRunner<'a> {
    /// Normal mode. Accepts a module and runs a set of validators and verifiers on it.
    NormalMode(Module, ChiselModuleSet<'a>),
    /// This variant is used when a creator module is present, and runs the translator/validator
    /// modules post-creation.
    CreateMode(&'a dyn ModuleCreator, ChiselModuleSet<'a>),
}

/// A list of Chisel modules to run. Translators are run before the validators by default.
pub struct ChiselModuleSet<'a> {
    /// Translators to be run. Translators are run before validators by default.
    translators: Vec<&'a dyn ModuleTranslator>,
    /// Validators to be run.
    validators: Vec<&'a dyn ModuleValidator>,
}

impl<'a> ChiselRunner<'a> {
    /// Instantiate a ruleset which operates on a module passed by the caller, and does not use
    /// creator modules.
    pub fn normal_mode(module: Module, chisel_mods: ChiselModuleSet<'a>) -> Self {
        ChiselRunner::NormalMode(module, chisel_mods)
    }

    /// Instantiate a ruleset which creates a module, rather than accepting one from the caller.
    pub fn create_mode(creator: &'a dyn ModuleCreator, chisel_mods: ChiselModuleSet<'a>) -> Self {
        ChiselRunner::CreateMode(creator, chisel_mods)
    }

    // FIXME: needs refactor
//    /// Run all rulesets and propagate errors.
//    pub fn run(&mut self) -> Result<(), ()> {
//        match self {
//            //TODO: why does only this variant need the ref pattern?
//            ChiselRunner::NormalMode(ref mut module, ref mut chisel_mods) => {
//                let translation_results = chisel_mods.run_translators(module);
//                // handle errors
//                let validation_results = chisel_mods.run_validators(module);
//                //TODO
//                Ok(())
//            }
//            ChiselRunner::CreateMode(creator, chisel_mods) => {
//                // TODO: Need to propagate errors here when it is implemented.
//                let mut module = creator.create().expect("creation failed");
//
//                // Run all the translators first.
//                let translation_results = chisel_mods.run_translators(&mut module);
//
//                // If any of the modules had an unrecoverable error, then propagate it.
//                if let Some(e) = translation_results.iter().find(|result| match result {
//                    Err(_) => true,
//                    Ok(_) => false,
//                }) {
//                    //TODO: proper error handling
//                    panic!("Translator module internal error");
//                }
//
//                // Validate.
//                let validation_results = chisel_mods.run_validators(&mut module);
//
//                // Propagate any errors here.
//                if let Some(e) = validation_results.iter().find(|result| match result {
//                    Err(_) => true,
//                    Ok(_) => false,
//                }) {
//                    //TODO: proper error handling
//                    panic!("Validator module internal error");
//                }
//
//                Ok(())
//            }
//        }
//    }
}

impl<'a> ChiselModuleSet<'a> {
    /// Construct an empty module set, defaulting not to translate in place.
    pub fn new() -> Self {
        ChiselModuleSet {
            translators: Vec::new(),
            validators: Vec::new(),
        }
    }

    /// Directly instantiate from arguments.
    pub fn with_modules(
        translator_mods: Vec<&'a dyn ModuleTranslator>,
        validator_mods: Vec<&'a dyn ModuleValidator>,
    ) -> Self {
        ChiselModuleSet {
            translators: translator_mods,
            validators: validator_mods,
        }
    }

    /// Take any type implementing ModuleTranslator and add it to the set.
    pub fn add_translator<T: ModuleTranslator>(&mut self, translator: &'a T) {
        self.translators.push(translator as &dyn ModuleTranslator);
    }

    /// Take any type implementing ModuleValidator and add it to the set.
    pub fn add_validator<V: ModuleValidator>(&mut self, validator: &'a V) {
        self.validators.push(validator as &dyn ModuleValidator);
    }

    /// Runs all the translators in the module set. On success, returns a Vec of results describing
    /// the status of the contained chisel modules' executions.
    pub fn run_translators(&mut self, mut module: &mut Module) -> Result<Vec<bool>, Vec<ModuleError>> {
        let (oks, errs): (Vec<Result<bool, ModuleError>>, Vec<Result<bool, ModuleError>>) = self.translators.iter()
            .map(|translator| {
                let result = translator.translate(&module);
                match result {
                    Ok(optional) => if let Some(new_module) = optional {
                        *module = new_module;
                        Ok(true)
                    } else {
                        Ok(false)
                    },
                    Err(e) => Err(e),
                }
            })
            .partition(|result| result.is_ok());

        // If there were any errors, propagate.
        if errs.len() > 0 {
            Err(errs.iter().map(|err| err.clone().unwrap_err()).collect())
        } else {
            Ok(oks.iter().map(|ok| ok.clone().unwrap()).collect())
        }
    }

    pub fn run_validators(&self, module: &Module) -> Result<Vec<bool>, Vec<ModuleError>> {
        // Iterate over the results of the validation and split them into two vectors of Ok and
        // Err.
        let (oks, errs): (Vec<Result<bool, ModuleError>>, Vec<Result<bool, ModuleError>>) = self.validators
            .iter()
            .map(|m| m.validate(&module))
            .partition(|result| result.is_ok());

        // If there were any errors, propagate.
        if errs.len() > 0 {
            Err(errs.iter().map(|err| err.clone().unwrap_err()).collect())
        } else {
            Ok(oks.iter().map(|ok| ok.clone().unwrap()).collect())
        }
    }
}
